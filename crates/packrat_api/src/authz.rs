use axum::Json;
use axum::http::StatusCode;
use packrat_application::AuthorizationQueryPort;
use packrat_domain::tenant::TenantId;
use packrat_domain::PermissionSlug;

use crate::dto::ErrorBody;
use crate::middleware::AuthSession;
use crate::state::AppState;

pub async fn ensure_tenant_permission(
    state: &AppState,
    session: &AuthSession,
    tenant_id: i64,
    slug: PermissionSlug,
) -> Result<(), (StatusCode, Json<ErrorBody>)> {
    match state.authorization.user_has_permission(
        session.user_id,
        TenantId::from(tenant_id),
        slug,
    )
    .await
    {
        Ok(true) => Ok(()),
        Ok(false) => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorBody::message("forbidden".to_string())),
        )),
        Err(e) => {
            tracing::error!(error = %e, "authorization query failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::message(
                    "authorization check failed".to_string(),
                )),
            ))
        }
    }
}
