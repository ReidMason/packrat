pub mod models;

pub use models::token_hash::TokenHash;

use crate::{asset::AssetTimestamp, user::UserId};

#[derive(Debug, Clone)]
pub struct UserSession {
    pub token_hash: TokenHash,
    pub user_id: UserId,
    pub expires_at: AssetTimestamp,
    pub create_at: AssetTimestamp,
}

impl UserSession {
    pub fn new(token_hash: TokenHash, user_id: UserId, duration: i64) -> Self {
        let expires_at =
            AssetTimestamp::from(chrono::Utc::now() + chrono::Duration::hours(duration));
        Self {
            token_hash,
            user_id,
            expires_at,
            create_at: AssetTimestamp::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        AssetTimestamp::now() > self.expires_at
    }
}
