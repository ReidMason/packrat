use std::sync::Arc;

use packrat_infrastructure::{PostgresItemCommand, PostgresItemQuery};

#[derive(Clone)]
pub struct AppState {
    pub command: Arc<PostgresItemCommand>,
    pub query: Arc<PostgresItemQuery>,
}
