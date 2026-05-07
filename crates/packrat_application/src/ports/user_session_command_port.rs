use async_trait::async_trait;
use packrat_domain::aggregates::user_session::{TokenHash, UserSession};

#[derive(Debug, PartialEq, Eq)]
pub enum UserSessionCommandError {
    Persist(String),
}

impl std::fmt::Display for UserSessionCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSessionCommandError::Persist(msg) => write!(f, "session persistence error: {msg}"),
        }
    }
}

impl std::error::Error for UserSessionCommandError {}

#[async_trait]
pub trait UserSessionCommandPort: Send + Sync {
    async fn save(&self, session: UserSession) -> Result<(), UserSessionCommandError>;
    async fn delete_by_token(&self, token: TokenHash) -> Result<(), UserSessionCommandError>;
}
