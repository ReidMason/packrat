use async_trait::async_trait;
use packrat_application::ItemCommandPort;
use packrat_domain::item::{Item, ItemId, ItemName};
use sqlx::PgPool;
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
    async fn create_item(&self, name: ItemName) -> Item {
        let id = sqlx::query_scalar!(
            "INSERT INTO items (name) VALUES ($1) RETURNING id",
            name.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .expect("insert item");
        Item::new(ItemId::new(id as u64), name, None)
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
