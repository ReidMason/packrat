use async_trait::async_trait;
use packrat_application::{ItemCommandPort, ItemPlacement};
use packrat_domain::inventory::InventoryId;
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
    async fn create_item(&self, name: ItemName, placement: ItemPlacement) -> Item {
        let (id, parent) = match placement {
            ItemPlacement::InLocation(location_id) => {
                let id = sqlx::query_scalar!(
                    "INSERT INTO items (name, location_id) VALUES ($1, $2) RETURNING id",
                    name.as_str(),
                    location_id.raw() as i64
                )
                .fetch_one(&self.pool)
                .await
                .expect("insert item");
                (id, InventoryId::Location(location_id))
            }
            ItemPlacement::InBucket(bucket_id) => {
                let id = sqlx::query_scalar!(
                    "INSERT INTO items (name, bucket_id) VALUES ($1, $2) RETURNING id",
                    name.as_str(),
                    bucket_id.raw() as i64
                )
                .fetch_one(&self.pool)
                .await
                .expect("insert item");
                (id, InventoryId::Bucket(bucket_id))
            }
        };
        Item::new(ItemId::new(id as u64), name, parent)
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
