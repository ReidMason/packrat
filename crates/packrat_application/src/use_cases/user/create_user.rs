use packrat_domain::user::{Email, PasswordHash, User};

use crate::{UserCommandPort, UserQueryPort};

pub struct CreateUserUseCase<C, Q> {
    user_command: C,
    user_query: Q,
}

impl<C, Q> CreateUserUseCase<C, Q>
where
    C: UserCommandPort,
    Q: UserQueryPort,
{
    pub fn new(user_command: C, user_query: Q) -> Self {
        Self {
            user_command,
            user_query,
        }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<User, String> {
        let email = Email::from(email);

        if self
            .user_query
            .get_user_by_email(&email)
            .await
            .map_err(|err| err.to_string())?
            .is_some()
        {
            return Err("Email already exists".to_string());
        }

        let password_hash = PasswordHash::generate(&password)?;

        self.user_command
            .create_user(email, password_hash)
            .await
            .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::UserQueryError;
    use crate::UserQueryPort;
    use crate::use_cases::user::create_user::CreateUserUseCase;
    use async_trait::async_trait;
    use packrat_domain::{
        asset::AssetTimestamp,
        user::{Email, PasswordHash, User, UserId},
    };

    use crate::{UserCommandError, UserCommandPort};

    fn some_user(email: Email) -> Option<User> {
        let id = UserId::from(1);
        let password_hash = PasswordHash::generate("test_password").unwrap();
        let timestamp = AssetTimestamp::static_for_tests();

        let user = User::new(id, email, password_hash, timestamp, timestamp);

        Some(user)
    }

    #[derive(Clone)]
    struct MockUserRepo {
        should_find_user: bool,
    }

    #[async_trait]
    impl UserCommandPort for MockUserRepo {
        async fn create_user(
            &self,
            email: Email,
            password_hash: PasswordHash,
        ) -> Result<User, UserCommandError> {
            let timestamp = AssetTimestamp::static_for_tests();
            let user = User::new(UserId::from(1), email, password_hash, timestamp, timestamp);

            Ok(user)
        }
    }

    #[async_trait]
    impl UserQueryPort for MockUserRepo {
        async fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, UserQueryError> {
            if self.should_find_user {
                return Ok(some_user(email.clone()));
            }
            Ok(None)
        }
        async fn get_user_by_id(&self, _id: UserId) -> Result<Option<User>, UserQueryError> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_create_success() {
        let mock = MockUserRepo {
            should_find_user: false,
        };
        let use_case = CreateUserUseCase::new(mock.clone(), mock);

        let result = use_case
            .execute("new@test.com".to_string(), "test_password".to_string())
            .await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email.as_str(), "new@test.com");
    }

    #[tokio::test]
    async fn test_create_fails_if_email_exists() {
        let mock = MockUserRepo {
            should_find_user: true,
        };
        let use_case = CreateUserUseCase::new(mock.clone(), mock);

        let result = use_case
            .execute("existing@test.com".to_string(), "test_password".to_string())
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email already exists");
    }
}
