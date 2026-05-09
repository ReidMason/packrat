use async_trait::async_trait;
use sqlx::PgPool;

use packrat_application::{UserCommandError, UserCommandPort, UserQueryError, UserQueryPort};
use packrat_domain::asset::AssetTimestamp;
use packrat_domain::user::{Email, PasswordHash, User, UserId};

#[derive(Clone)]
pub struct PostgresUserCommand {
    pool: PgPool,
}

impl PostgresUserCommand {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserCommandPort for PostgresUserCommand {
    async fn create_user(
        &self,
        email: Email,
        password_hash: PasswordHash,
    ) -> Result<User, UserCommandError> {
        let normalized = email.as_str().trim().to_lowercase();
        let result = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash, created, updated
            "#,
            normalized,
            password_hash.as_str(),
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => Ok(User::new(
                UserId::from(row.id),
                Email::from(row.email),
                PasswordHash::from_hashed(&row.password_hash),
                AssetTimestamp::from(row.created),
                AssetTimestamp::from(row.updated),
            )),
            Err(e) => {
                if let Some(db) = e.as_database_error() {
                    if db.code().as_deref() == Some("23505") {
                        return Err(UserCommandError::DuplicateEmail);
                    }
                }
                Err(UserCommandError::Persist(e.to_string()))
            }
        }
    }
}

#[derive(Clone)]
pub struct PostgresUserQuery {
    pool: PgPool,
}

impl PostgresUserQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserQueryPort for PostgresUserQuery {
    async fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, UserQueryError> {
        let normalized = email.as_str().trim().to_lowercase();

        let result = sqlx::query!(
            r#"
            SELECT id, email, password_hash, created, updated
            FROM users
            WHERE email = $1
            "#,
            normalized
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => Ok(Some(User::new(
                UserId::from(row.id),
                Email::from(row.email),
                PasswordHash::from_hashed(&row.password_hash),
                AssetTimestamp::from(row.created),
                AssetTimestamp::from(row.updated),
            ))),
            Ok(None) => Ok(None),
            Err(e) => Err(UserQueryError::Infrastructure(e.to_string())),
        }
    }

    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>, UserQueryError> {
        let id_raw = i64::from(id);
        let result = sqlx::query!(
            r#"
            SELECT id, email, password_hash, created, updated
            FROM users
            WHERE id = $1
            "#,
            id_raw
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => Ok(Some(User::new(
                UserId::from(row.id),
                Email::from(row.email),
                PasswordHash::from_hashed(&row.password_hash),
                AssetTimestamp::from(row.created),
                AssetTimestamp::from(row.updated),
            ))),
            Ok(None) => Ok(None),
            Err(e) => Err(UserQueryError::Infrastructure(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_user_inserts_row(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(
                Email::from("hello@example.com"),
                PasswordHash::generate("test_password").unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(user.email.as_str(), "hello@example.com");
        assert!(i64::from(user.id) > 0);

        let count = sqlx::query_scalar!(
            r#"SELECT COUNT(*)::bigint AS "count!" FROM users WHERE email = $1"#,
            "hello@example.com",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test]
    async fn create_user_duplicate_email(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool);
        cmd.create_user(
            Email::from("dup@example.com"),
            PasswordHash::generate("test_password").unwrap(),
        )
        .await
        .unwrap();
        let err = cmd
            .create_user(
                Email::from("dup@example.com"),
                PasswordHash::generate("test_password").unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(err, UserCommandError::DuplicateEmail);
    }
}
