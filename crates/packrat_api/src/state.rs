use std::sync::Arc;

use packrat_infrastructure::{
    PostgresAssetCommand, PostgresAssetQuery, PostgresAuthorizationQuery, PostgresReadiness,
    PostgresTags, PostgresTenantCommand, PostgresTenantQuery, PostgresUserCommand,
    PostgresUserQuery, PostgresUserSessionCommand, PostgresUserSessionQuery,
};

#[derive(Clone)]
pub struct AppState {
    pub readiness: PostgresReadiness,
    pub command: Arc<PostgresAssetCommand>,
    pub query: Arc<PostgresAssetQuery>,
    pub user_command: Arc<PostgresUserCommand>,
    pub user_query: Arc<PostgresUserQuery>,
    pub user_session_command: Arc<PostgresUserSessionCommand>,
    pub user_session_query: Arc<PostgresUserSessionQuery>,
    pub tenant_command: Arc<PostgresTenantCommand>,
    pub tenant_query: Arc<PostgresTenantQuery>,
    pub authorization: Arc<PostgresAuthorizationQuery>,
    pub tags: Arc<PostgresTags>,
}
