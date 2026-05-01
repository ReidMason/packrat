use async_trait::async_trait;
use sqlx::PgPool;

use packrat_application::ReadinessPort;

use crate::postgres::ping_database;

#[derive(Clone)]
pub struct PostgresReadiness {
    pool: PgPool,
}

impl PostgresReadiness {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadinessPort for PostgresReadiness {
    async fn check_database(&self) -> Result<(), String> {
        ping_database(&self.pool).await.map_err(|e| e.to_string())
    }
}
