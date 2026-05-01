use serde::Serialize;

/// **DTO** — payload for `GET /api/health` inside [`SuccessBody`](super::SuccessBody).
#[derive(Serialize)]
pub struct HealthDto {
    pub status: &'static str,
}

impl HealthDto {
    pub fn ok() -> Self {
        Self { status: "ok" }
    }
}
