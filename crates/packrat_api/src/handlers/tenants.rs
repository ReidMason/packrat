use axum::Json;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use packrat_application::{TenantCommandError, create_tenant, list_tenants_for_user};
use packrat_domain::tenant::TenantName;

use crate::dto::{CreateTenantDto, ErrorBody, SuccessBody, TenantDto};
use crate::middleware::AuthSession;
use crate::state::AppState;

pub async fn create_tenant_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Json(body): Json<CreateTenantDto>,
) -> Result<(StatusCode, Json<SuccessBody<TenantDto>>), (StatusCode, Json<ErrorBody>)> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("name must not be empty")),
        ));
    }

    match create_tenant(
        state.tenant_command.as_ref(),
        session.user_id,
        TenantName::from(name),
    )
    .await {
        Ok(tenant) => Ok((
            StatusCode::CREATED,
            Json(SuccessBody::new(TenantDto::from_tenant(tenant))),
        )),
        Err(TenantCommandError::Persist(msg)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody::message(msg)),
        )),
    }
}

pub async fn list_my_tenants_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
) -> Result<Json<SuccessBody<Vec<TenantDto>>>, (StatusCode, Json<ErrorBody>)> {
    let tenants = list_tenants_for_user(state.tenant_query.as_ref(), session.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::message(e)),
            )
        })?;
    Ok(Json(SuccessBody::new(
        tenants.into_iter().map(TenantDto::from_tenant).collect(),
    )))
}
