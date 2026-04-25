use async_trait::async_trait;
use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::item::{Entity, EntityId, EntityName};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

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
