use std::sync::Arc;

use packrat_infrastructure::{PostgresAssetCommand, PostgresAssetQuery, PostgresReadiness};

#[derive(Clone)]
pub struct AppState {
    pub readiness: PostgresReadiness,
    pub command: Arc<PostgresAssetCommand>,
    pub query: Arc<PostgresAssetQuery>,
}
