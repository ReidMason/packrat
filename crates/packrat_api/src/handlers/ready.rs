use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use packrat_application::check_readiness;

use crate::dto::{ErrorBody, ReadyDto, SuccessBody};
use crate::state::AppState;

pub async fn ready_handler(
    State(state): State<AppState>,
) -> Result<Json<SuccessBody<ReadyDto>>, (StatusCode, Json<ErrorBody>)> {
    check_readiness(&state.readiness).await.map_err(|e| {
        tracing::warn!(error = %e, "readiness check failed");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorBody::message(format!("database unavailable: {e}"))),
        )
    })?;
    Ok(Json(SuccessBody::new(ReadyDto::ok())))
}
