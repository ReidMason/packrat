use async_trait::async_trait;
use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::entity::{Entity, EntityId, EntityName};
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
        // TODO: This should be updated to entities over items
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO items (name, parent_id) VALUES ($1, $2) RETURNING id",
            name.as_str(),
            parent.map(i64::from) as Option<i64>,
        )
        .fetch_one(&self.pool)
        .await
        .expect("insert item");
        Entity::new(EntityId::from(id), name, parent)
    }

    async fn delete_entity(&self, id: EntityId) -> Result<(), String> {
        let mut transaction = self.pool.begin().await.map_err(|err| err.to_string())?;

        // WARN: This should probably set parent_id to the root id over NULL?
        sqlx::query!(
            "UPDATE items SET parent_id = NULL WHERE parent_id = $1",
            i64::from(id)
        )
        .execute(transaction.as_mut())
        .await
        .map_err(|err| err.to_string())?;

        let result = sqlx::query!("DELETE FROM items WHERE id = $1", i64::from(id))
            .execute(transaction.as_mut())
            .await
            .map_err(|err| err.to_string())?;

        transaction.commit().await.map_err(|err| err.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Entity not found".to_string());
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
        let row = sqlx::query("SELECT id, name, parent_id FROM items WHERE id = $1")
            .bind(i64::from(id))
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()?;

        let id: i64 = row.try_get("id").ok()?;
        let name: String = row.try_get("name").ok()?;
        let parent_id: Option<i64> = row.try_get("parent_id").ok()?;
        Some(Entity::new(
            EntityId::from(id),
            EntityName::from(name),
            parent_id.map(EntityId::from),
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

#[cfg(test)]
mod integration_tests {
    use super::*;
    use packrat_domain::entity::EntityName;

    #[sqlx::test]
    async fn test_delete_item_orphans_children(pool: PgPool) {
        let command = PostgresItemCommand::new(pool.clone());
        let query = PostgresItemQuery::new(pool);

        let parent_name = EntityName::from("Toolbox");
        let child_name = EntityName::from("Spanner");
        
        let parent = command.create_item(parent_name, None).await;
        let child = command.create_item(child_name, Some(parent.id)).await;

        let result = command.delete_entity(parent.id).await;

        assert!(result.is_ok(), "Delete operation failed");

        let fetched_parent = query.get_item_by_id(parent.id).await;
        assert!(fetched_parent.is_none(), "Parent should have been deleted");

        let fetched_child = query.get_item_by_id(child.id).await
            .expect("Child should still exist");
        
        assert_eq!(fetched_child.parent, None, "Child should have been orphaned (parent_id = NULL)");
    }

    #[sqlx::test]
    async fn test_delete_non_existent_item_returns_error(pool: PgPool) {
        let command = PostgresItemCommand::new(pool);
        let fake_id = EntityId::from(9999);

        let result = command.delete_entity(fake_id).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
