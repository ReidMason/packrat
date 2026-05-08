use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::Row;

use packrat_application::{
    UserSessionCommandError, UserSessionCommandPort, UserSessionQueryError, UserSessionQueryPort,
};
use packrat_domain::asset::AssetTimestamp;
use packrat_domain::aggregates::user_session::{TokenHash, UserSession};
use packrat_domain::user::UserId;

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

pub struct PostgresUserSessionQuery {
    pool: PgPool,
}

impl PostgresUserSessionQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSessionQueryPort for PostgresUserSessionQuery {
    async fn get_by_token(
        &self,
        token: &TokenHash,
    ) -> Result<Option<UserSession>, UserSessionQueryError> {
        let row = sqlx::query(
            r#"
            SELECT user_id, token_hash, created_at, expires_at, revoked_at
            FROM sessions
            WHERE token_hash = $1
              AND expires_at > NOW()
              AND revoked_at IS NULL
            "#,
        )
        .bind(token.as_bytes())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let user_id: i64 = row
            .try_get("user_id")
            .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;
        let token_hash_bytes: Vec<u8> = row
            .try_get("token_hash")
            .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;
        let token_hash = TokenHash::from_sha256_digest(token_hash_bytes).ok_or_else(|| {
            UserSessionQueryError::Infrastructure("invalid token_hash length".into())
        })?;

        let created_at: DateTime<Utc> = row
            .try_get("created_at")
            .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;
        let expires_at: DateTime<Utc> = row
            .try_get("expires_at")
            .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;
        let revoked_at: Option<DateTime<Utc>> = row
            .try_get("revoked_at")
            .map_err(|e| UserSessionQueryError::Infrastructure(e.to_string()))?;

        Ok(Some(UserSession {
            token_hash,
            user_id: UserId::from(user_id),
            expires_at: AssetTimestamp::from(expires_at),
            created_at: AssetTimestamp::from(created_at),
            revoked_at: revoked_at.map(AssetTimestamp::from),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_application::{UserCommandPort, UserSessionQueryPort};
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

    #[sqlx::test]
    async fn get_by_token_returns_session(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("query-session@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _) = TokenHash::generate();
        let session = UserSession::new(hash.clone(), user.id, 24);

        let write = PostgresUserSessionCommand::new(pool.clone());
        write.save(session).await.unwrap();

        let read = PostgresUserSessionQuery::new(pool);
        let found = read.get_by_token(&hash).await.unwrap().unwrap();

        assert_eq!(found.user_id, user.id);
        assert_eq!(found.token_hash.as_bytes(), hash.as_bytes());
        assert!(found.revoked_at.is_none());
    }

    #[sqlx::test]
    async fn get_by_token_returns_none_when_missing(pool: PgPool) {
        let (hash, _) = TokenHash::generate();
        let read = PostgresUserSessionQuery::new(pool);
        assert!(read.get_by_token(&hash).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn get_by_token_returns_none_after_delete(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("query-del@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _) = TokenHash::generate();
        let hash_clone = hash.clone();
        let session = UserSession::new(hash, user.id, 1);

        let write = PostgresUserSessionCommand::new(pool.clone());
        write.save(session).await.unwrap();

        assert!(PostgresUserSessionQuery::new(pool.clone())
            .get_by_token(&hash_clone)
            .await
            .unwrap()
            .is_some());

        write.delete_by_token(hash_clone.clone()).await.unwrap();

        assert!(PostgresUserSessionQuery::new(pool)
            .get_by_token(&hash_clone)
            .await
            .unwrap()
            .is_none());
    }

    #[sqlx::test]
    async fn get_by_token_returns_none_when_expired(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("query-expired@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _) = TokenHash::generate();
        let session = UserSession::new(hash.clone(), user.id, 24);

        let write = PostgresUserSessionCommand::new(pool.clone());
        write.save(session).await.unwrap();

        sqlx::query(
            r#"
            UPDATE sessions
            SET created_at = NOW() - INTERVAL '3 hours',
                expires_at = NOW() - INTERVAL '1 second'
            WHERE token_hash = $1
            "#,
        )
        .bind(hash.as_bytes())
        .execute(&pool)
        .await
        .unwrap();

        assert!(PostgresUserSessionQuery::new(pool)
            .get_by_token(&hash)
            .await
            .unwrap()
            .is_none());
    }

    #[sqlx::test]
    async fn get_by_token_returns_none_when_revoked(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("query-revoked@example.com"),
                PasswordHash::generate("pw").unwrap(),
            )
            .await
            .unwrap();

        let (hash, _) = TokenHash::generate();
        let session = UserSession::new(hash.clone(), user.id, 24);

        let write = PostgresUserSessionCommand::new(pool.clone());
        write.save(session).await.unwrap();

        sqlx::query("UPDATE sessions SET revoked_at = NOW() WHERE token_hash = $1")
            .bind(hash.as_bytes())
            .execute(&pool)
            .await
            .unwrap();

        assert!(PostgresUserSessionQuery::new(pool)
            .get_by_token(&hash)
            .await
            .unwrap()
            .is_none());
    }
}
