use serde::Serialize;

/// **DTO** — payload for `GET /api/ready` inside [`SuccessBody`](super::SuccessBody).
#[derive(Serialize)]
pub struct ReadyDto {
    pub status: &'static str,
    pub database: &'static str,
}

impl ReadyDto {
    pub fn ok() -> Self {
        Self {
            status: "ready",
            database: "ok",
        }
    }
}
