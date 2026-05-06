use async_trait::async_trait;
use packrat_domain::user::{Email, User, UserId};

#[derive(Debug)]
pub enum UserQueryError {
    NotFound,
    Infrastructer(String),
}

#[async_trait]
pub trait UserQueryPort: Send + Sync {
    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>, UserQueryError>;
    async fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, UserQueryError>;
}
