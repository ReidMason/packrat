use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use packrat_application::{TenantCommandError, create_tenant};
use packrat_domain::tenant::TenantName;

use crate::dto::{CreateTenantDto, ErrorBody, SuccessBody, TenantDto};
use crate::state::AppState;

pub async fn create_tenant_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateTenantDto>,
) -> Result<(StatusCode, Json<SuccessBody<TenantDto>>), (StatusCode, Json<ErrorBody>)> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("name must not be empty")),
        ));
    }

    match create_tenant(state.tenant_command.as_ref(), TenantName::from(name)).await {
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
