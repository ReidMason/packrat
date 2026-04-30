use axum::Json;

use crate::dto::{HealthDto, SuccessBody};

pub async fn health_handler() -> Json<SuccessBody<HealthDto>> {
    Json(SuccessBody::new(HealthDto::ok()))
}
