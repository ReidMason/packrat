use async_trait::async_trait;
use packrat_domain::user::{Email, User, UserId};

#[derive(Debug)]
pub enum UserQueryError {
    NotFound,
    Infrastructure(String),
}

impl std::fmt::Display for UserQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserQueryError::NotFound => write!(f, "session not found"),
            UserQueryError::Infrastructure(msg) => write!(f, "database error: {msg}"),
        }
    }
}

impl std::error::Error for UserQueryError {}

#[async_trait]
pub trait UserQueryPort: Send + Sync {
    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>, UserQueryError>;
    async fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, UserQueryError>;
}
