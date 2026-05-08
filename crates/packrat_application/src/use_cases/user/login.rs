use packrat_domain::{
    user::Email,
    user_session::{TokenHash, UserSession},
};

use crate::{UserQueryPort, ports::UserSessionCommandPort};

#[derive(Debug)]
pub struct LoginResponse {
    pub user_id: i64,
    pub token: String,
}

pub struct LoginUseCase<UQ, SC> {
    user_query: UQ,
    session_command: SC,
}

impl<UQ, SC> LoginUseCase<UQ, SC>
where
    UQ: UserQueryPort,
    SC: UserSessionCommandPort,
{
    pub fn new(user_query: UQ, session_command: SC) -> Self {
        Self {
            user_query,
            session_command,
        }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<LoginResponse, String> {
        let email = Email::from(email);

        let user = self
            .user_query
            .get_user_by_email(&email)
            .await
            .map_err(|err| err.to_string())?
            .ok_or_else(|| "Invalid email".to_string())?;

        if !user.password.verify(&password) {
            return Err("Invalid password".to_string());
        }

        let (token_hash, token) = TokenHash::generate();

        let session = UserSession::new(token_hash, user.id, 24);

        self.session_command
            .save(session)
            .await
            .map_err(|err| err.to_string())?;

        let login_response = LoginResponse {
            user_id: i64::from(user.id),
            token,
        };

        Ok(login_response)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::{
        asset::AssetTimestamp,
        user::{Email, PasswordHash, User, UserId},
        user_session::{TokenHash, UserSession},
    };

    use crate::{
        UserQueryError, UserQueryPort,
        ports::{UserSessionCommandError, UserSessionCommandPort},
        use_cases::user::login::LoginUseCase,
    };

    #[derive(Clone)]
    struct MockLoginRepo {
        user_exists: bool,
        correct_password: bool,
    }

    #[async_trait]
    impl UserQueryPort for MockLoginRepo {
        async fn get_user_by_email(&self, email: &Email) -> Result<Option<User>, UserQueryError> {
            if !self.user_exists {
                return Ok(None);
            }

            let hash = if self.correct_password {
                PasswordHash::generate("test_password").unwrap()
            } else {
                PasswordHash::generate("different_test_password").unwrap()
            };

            let timestamp = AssetTimestamp::static_for_tests();
            Ok(Some(User::new(
                UserId::from(1),
                email.clone(),
                hash,
                timestamp,
                timestamp,
            )))
        }
        async fn get_user_by_id(&self, _id: UserId) -> Result<Option<User>, UserQueryError> {
            Ok(None)
        }
    }

    #[async_trait]
    impl UserSessionCommandPort for MockLoginRepo {
        async fn save(&self, _session: UserSession) -> Result<(), UserSessionCommandError> {
            Ok(())
        }
        async fn delete_by_token(&self, _hash: TokenHash) -> Result<(), UserSessionCommandError> {
            Ok(())
        }
        async fn delete_all_for_user(
            &self,
            _user_id: UserId,
        ) -> Result<(), UserSessionCommandError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let mock = MockLoginRepo {
            user_exists: true,
            correct_password: true,
        };
        let use_case = LoginUseCase::new(mock.clone(), mock);

        let result = use_case
            .execute("test@test.com".into(), "test_password".into())
            .await;

        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(!res.token.is_empty());
        assert_eq!(res.user_id, 1);
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let mock = MockLoginRepo {
            user_exists: true,
            correct_password: false,
        };
        let use_case = LoginUseCase::new(mock.clone(), mock);

        let result = use_case
            .execute("test@test.com".into(), "test_password".into())
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid password");
    }
}
