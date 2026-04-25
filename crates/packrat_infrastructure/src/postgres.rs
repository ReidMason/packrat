use async_trait::async_trait;
use packrat_application::ItemCommandPort;
use packrat_domain::item::{Entity, EntityId, EntityName};
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
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
        let (id, _) = match parent {
            Some(parent) => {
                let id = sqlx::query_scalar!(
                    "INSERT INTO items (name, parent_id) VALUES ($1, $2) RETURNING id",
                    name.as_str(),
                    i64::from(parent)
                )
                .fetch_one(&self.pool)
                .await
                .expect("insert item");
                (id, parent)
            }
            None => {
                todo!()
            }
        };
        Entity::new(EntityId::from(id), name, parent)
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
