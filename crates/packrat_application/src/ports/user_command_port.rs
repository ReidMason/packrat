use async_trait::async_trait;
use packrat_domain::user::{Email, User};

#[derive(Debug, PartialEq, Eq)]
pub enum UserCommandError {
    DuplicateEmail,
    Persist(String),
}

impl std::fmt::Display for UserCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserCommandError::DuplicateEmail => write!(f, "email already registered"),
            UserCommandError::Persist(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for UserCommandError {}

#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn create_user(&self, email: Email) -> Result<User, UserCommandError>;
}
