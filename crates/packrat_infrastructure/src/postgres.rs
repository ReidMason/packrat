use async_trait::async_trait;
use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::entity::EntityTimestamp;
use packrat_domain::entity::{Entity, EntityId, EntityName};
use packrat_domain::models::partial_entity::PartialEntity;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

pub struct PostgresItemCommand {
    pool: PgPool,
}

impl PostgresItemCommand {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemCommandPort for PostgresItemCommand {
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
        let created = EntityTimestamp::now();
        let deleted = None;
        // TODO: This should be updated to entities over items
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO items (name, parent_id, created, deleted) VALUES ($1, $2, $3, $4) RETURNING id",
            name.as_str(),
            parent.map(i64::from) as Option<i64>,
            chrono::DateTime::from(created),
            deleted.map(chrono::DateTime::from) as Option<chrono::DateTime<chrono::Utc>>,
        )
        .fetch_one(&self.pool)
        .await
        .expect("insert item");

        Entity::new(EntityId::from(id), name, parent, created, deleted)
    }

    async fn update_entity(&self, id: EntityId, changes: PartialEntity) -> Result<(), String> {
        let current_row = sqlx::query!(
            "SELECT name, parent_id FROM items WHERE id = $1 AND deleted IS NULL",
            i64::from(id)
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Entity {} not found", i64::from(id)))?;

        let name = match changes.name {
            Some(name) => String::from(name),
            None => current_row.name,
        };

        let parent = match changes.parent {
            Some(new_parent) => new_parent.map(i64::from),
            None => current_row.parent_id,
        };

        let result = sqlx::query!(
            "UPDATE items SET name = $1, parent_id = $2 WHERE id = $3",
            name,
            parent,
            i64::from(id)
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!("Entity with ID {} not found", i64::from(id)));
        }

        Ok(())
    }

    async fn delete_entity(&self, id: EntityId) -> Result<(), String> {
        let is_a_parent = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM items WHERE parent_id = $1 AND deleted IS NULL)",
            i64::from(id)
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())?;

        if is_a_parent.unwrap_or(false) {
            return Err("Cannot Delete: Entity has active children".into());
        }

        let result = sqlx::query!(
            "UPDATE items SET deleted = NOW() WHERE id = $1 AND deleted IS NULL",
            i64::from(id),
        )
        .execute(&self.pool)
        .await
        .map_err(|err| err.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!("Item with ID {} not found", i64::from(id)));
        }

        Ok(())
    }
}

pub struct PostgresItemQuery {
    pool: PgPool,
}

impl PostgresItemQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemQueryPort for PostgresItemQuery {
    async fn get_item_by_id(&self, id: EntityId) -> Option<Entity> {
        let row =
            sqlx::query("SELECT id, name, parent_id, created, deleted FROM items WHERE id = $1")
                .bind(i64::from(id))
                .fetch_optional(&self.pool)
                .await
                .ok()
                .flatten()?;

        let id: i64 = row.try_get("id").ok()?;
        let name: String = row.try_get("name").ok()?;
        let parent_id: Option<i64> = row.try_get("parent_id").ok()?;
        let created: chrono::DateTime<chrono::Utc> = row
            .try_get::<'_, chrono::DateTime<chrono::Utc>, _>("created")
            .ok()?;
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row
            .try_get::<'_, Option<chrono::DateTime<chrono::Utc>>, _>("deleted")
            .ok()?;

        Some(Entity::new(
            EntityId::from(id),
            EntityName::from(name),
            parent_id.map(EntityId::from),
            EntityTimestamp::from(created),
            deleted.map(EntityTimestamp::from),
        ))
    }
}

pub async fn connect_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn ping_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod postgres_tests {
    use super::*;
    use packrat_domain::entity::EntityName;
    use sqlx::Row;

    #[sqlx::test]
    async fn test_delete_entity_errors_when_is_parent(pool: PgPool) {
        let command = PostgresItemCommand::new(pool.clone());

        let parent = command.create_item(EntityName::from("Parent"), None).await;

        let _child = command
            .create_item(EntityName::from("Child"), Some(parent.id))
            .await;

        let result = command.delete_entity(parent.id).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot Delete: Entity has active children"
        );

        let row = sqlx::query("SELECT deleted FROM items WHERE id = $1")
            .bind(i64::from(parent.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row.try_get("deleted").unwrap();

        assert!(deleted.is_none());
    }

    #[sqlx::test]
    async fn test_delete_non_existent_item_returns_error(pool: PgPool) {
        let command = PostgresItemCommand::new(pool);
        let fake_id = EntityId::from(999);

        let result = command.delete_entity(fake_id).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[sqlx::test]
    async fn test_delete_item_successfully(pool: PgPool) {
        let command = PostgresItemCommand::new(pool.clone());
        let item = command.create_item(EntityName::from("Target"), None).await;

        let result = command.delete_entity(item.id).await;

        assert!(result.is_ok());

        let row = sqlx::query("SELECT deleted FROM items WHERE id = $1")
            .bind(i64::from(item.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row.try_get("deleted").unwrap();

        assert!(deleted.is_some());
    }

    #[sqlx::test]
    async fn test_update_item_name_only(pool: PgPool) {
        let command = PostgresItemCommand::new(pool.clone());
        let item = command
            .create_item(EntityName::from("Old Name"), None)
            .await;

        let changes = PartialEntity {
            name: Some(EntityName::from("New Name")),
            parent: None, // No change to parent
        };

        let result = command.update_entity(item.id, changes).await;
        assert!(result.is_ok());

        let row = sqlx::query("SELECT name FROM items WHERE id = $1")
            .bind(i64::from(item.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let name: String = row.try_get("name").unwrap();

        assert_eq!(name, "New Name");
    }
}
