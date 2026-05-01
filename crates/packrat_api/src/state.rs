use std::sync::Arc;

use packrat_infrastructure::{PostgresItemCommand, PostgresItemQuery, PostgresReadiness};

#[derive(Clone)]
pub struct AppState {
    pub readiness: PostgresReadiness,
    pub command: Arc<PostgresItemCommand>,
    pub query: Arc<PostgresItemQuery>,
}
