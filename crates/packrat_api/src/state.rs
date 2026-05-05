use std::sync::Arc;

use packrat_infrastructure::{
    PostgresAssetCommand, PostgresAssetQuery, PostgresReadiness, PostgresTenantCommand,
    PostgresUserCommand,
};

#[derive(Clone)]
pub struct AppState {
    pub readiness: PostgresReadiness,
    pub command: Arc<PostgresAssetCommand>,
    pub query: Arc<PostgresAssetQuery>,
    pub user_command: Arc<PostgresUserCommand>,
    pub tenant_command: Arc<PostgresTenantCommand>,
}
