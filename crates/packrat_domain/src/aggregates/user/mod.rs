pub mod models;

pub use models::{email::Email, password_hash::PasswordHash, user_id::UserId};

use crate::asset::AssetTimestamp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password: PasswordHash,
    pub created: AssetTimestamp,
    pub updated: AssetTimestamp,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        password: PasswordHash,
        created: AssetTimestamp,
        updated: AssetTimestamp,
    ) -> Self {
        Self {
            id,
            email,
            password,
            created,
            updated,
        }
    }
}
