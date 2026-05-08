pub mod models;

pub use models::token_hash::TokenHash;

use crate::{asset::AssetTimestamp, user::UserId};

#[derive(Debug, Clone)]
pub struct UserSession {
    pub token_hash: TokenHash,
    pub user_id: UserId,
    pub expires_at: AssetTimestamp,
    pub created_at: AssetTimestamp,
    pub revoked_at: Option<AssetTimestamp>,
}

impl UserSession {
    pub fn new(token_hash: TokenHash, user_id: UserId, duration: i64) -> Self {
        let expires_at =
            AssetTimestamp::from(chrono::Utc::now() + chrono::Duration::hours(duration));
        Self {
            token_hash,
            user_id,
            expires_at,
            created_at: AssetTimestamp::now(),
            revoked_at: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        AssetTimestamp::now() > self.expires_at
    }
}
