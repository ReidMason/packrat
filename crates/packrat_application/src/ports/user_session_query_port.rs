use async_trait::async_trait;
use packrat_domain::aggregates::user_session::{TokenHash, UserSession};

#[derive(Debug)]
pub enum UserSessionQueryError {
    NotFound,
    Infrastructure(String),
}

impl std::fmt::Display for UserSessionQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSessionQueryError::NotFound => write!(f, "session not found"),
            UserSessionQueryError::Infrastructure(msg) => write!(f, "database error: {msg}"),
        }
    }
}

impl std::error::Error for UserSessionQueryError {}

#[async_trait]
pub trait UserSessionQueryPort: Send + Sync {
    async fn find_by_hash(
        &self,
        hash: &TokenHash,
    ) -> Result<Option<UserSession>, UserSessionQueryError>;
}
