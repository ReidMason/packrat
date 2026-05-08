use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use packrat_application::{UserSessionCommandError, UserSessionCommandPort};
use packrat_domain::aggregates::user_session::{TokenHash, UserSession};

pub struct PostgresUserSessionCommand {
    pool: PgPool,
}

impl PostgresUserSessionCommand {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSessionCommandPort for PostgresUserSessionCommand {
    async fn save(&self, session: UserSession) -> Result<(), UserSessionCommandError> {
        let user_id = i64::from(session.user_id);
        let token_hash: Vec<u8> = session.token_hash.clone().into();
        let created: DateTime<Utc> = session.created_at.into();
        let expires_at: DateTime<Utc> = session.expires_at.into();
        let revoked_at: Option<DateTime<Utc>> = session.revoked_at.map(Into::into);

        sqlx::query(
            r#"
            INSERT INTO sessions (user_id, token_hash, created_at, expires_at, revoked_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user_id)
        .bind(token_hash.as_slice())
        .bind(created)
        .bind(expires_at)
        .bind(revoked_at)
        .execute(&self.pool)
        .await
        .map_err(|e| UserSessionCommandError::Persist(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_token(&self, token: TokenHash) -> Result<(), UserSessionCommandError> {
        let token_hash: Vec<u8> = token.into();

        sqlx::query(r#"DELETE FROM sessions WHERE token_hash = $1"#)
            .bind(token_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| UserSessionCommandError::Persist(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_application::UserCommandPort;
    use packrat_domain::user::{Email, PasswordHash};
    use sqlx::Row;

    use crate::postgres_user::PostgresUserCommand;

    #[sqlx::test]
    async fn save_session_inserts_row(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("session-user@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _raw) = TokenHash::generate();
        let session = UserSession::new(hash, user.id, 24);

        let sessions = PostgresUserSessionCommand::new(pool.clone());
        sessions.save(session.clone()).await.unwrap();

        let row = sqlx::query(
            "SELECT user_id, expires_at, revoked_at FROM sessions WHERE token_hash = $1",
        )
        .bind(Vec::<u8>::from(session.token_hash.clone()))
        .fetch_one(&pool)
        .await
        .unwrap();

        let uid: i64 = row.try_get("user_id").unwrap();
        assert_eq!(uid, i64::from(user.id));
        let revoked: Option<DateTime<Utc>> = row.try_get("revoked_at").unwrap();
        assert!(revoked.is_none());
        let exp: DateTime<Utc> = row.try_get("expires_at").unwrap();
        assert!(exp > DateTime::<Utc>::from(session.created_at));
    }

    #[sqlx::test]
    async fn delete_by_token_removes_row(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("del-session@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _) = TokenHash::generate();
        let hash_clone = hash.clone();
        let session = UserSession::new(hash, user.id, 1);

        let sessions = PostgresUserSessionCommand::new(pool.clone());
        sessions.save(session).await.unwrap();

        sessions.delete_by_token(hash_clone).await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
    }
}
