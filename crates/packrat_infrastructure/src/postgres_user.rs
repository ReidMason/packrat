use async_trait::async_trait;
use sqlx::{PgPool, Row};

use packrat_application::{UserCommandError, UserCommandPort};
use packrat_domain::entity::EntityTimestamp;
use packrat_domain::user::{Email, User, UserId};

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
    async fn create_user(&self, email: Email) -> Result<User, UserCommandError> {
        let normalized = email.as_str().trim().to_lowercase();
        let result = sqlx::query(
            r#"
            INSERT INTO users (email)
            VALUES ($1)
            RETURNING id, email, created, updated
            "#,
        )
        .bind(&normalized)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let id: i64 = row
                    .try_get("id")
                    .map_err(|e| UserCommandError::Persist(e.to_string()))?;
                let email_db: String = row
                    .try_get("email")
                    .map_err(|e| UserCommandError::Persist(e.to_string()))?;
                let created: chrono::DateTime<chrono::Utc> = row
                    .try_get("created")
                    .map_err(|e| UserCommandError::Persist(e.to_string()))?;
                let updated: chrono::DateTime<chrono::Utc> = row
                    .try_get("updated")
                    .map_err(|e| UserCommandError::Persist(e.to_string()))?;
                Ok(User::new(
                    UserId::from(id),
                    Email::from(email_db),
                    EntityTimestamp::from(created),
                    EntityTimestamp::from(updated),
                ))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_user_inserts_row(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool.clone());
        let user = cmd
            .create_user(Email::from("hello@example.com"))
            .await
            .unwrap();
        assert_eq!(user.email.as_str(), "hello@example.com");
        assert!(i64::from(user.id) > 0);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind("hello@example.com")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test]
    async fn create_user_duplicate_email(pool: PgPool) {
        let cmd = PostgresUserCommand::new(pool);
        cmd.create_user(Email::from("dup@example.com"))
            .await
            .unwrap();
        let err = cmd
            .create_user(Email::from("dup@example.com"))
            .await
            .unwrap_err();
        assert_eq!(err, UserCommandError::DuplicateEmail);
    }
}
