use packrat_domain::user::{Email, User};

use crate::ports::{UserCommandError, UserCommandPort};

pub async fn execute(port: &impl UserCommandPort, email: Email) -> Result<User, UserCommandError> {
    port.create_user(email).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockUserCommand;

    #[async_trait]
    impl UserCommandPort for MockUserCommand {
        async fn create_user(&self, email: Email) -> Result<User, UserCommandError> {
            Ok(User::new(
                packrat_domain::user::UserId::from(1),
                email,
                packrat_domain::entity::EntityTimestamp::static_for_tests(),
                packrat_domain::entity::EntityTimestamp::static_for_tests(),
            ))
        }
    }

    #[tokio::test]
    async fn execute_delegates_to_port() {
        let port = MockUserCommand;
        let user = execute(&port, Email::from("a@b.co")).await.unwrap();
        assert_eq!(user.email.as_str(), "a@b.co");
        assert_eq!(i64::from(user.id), 1);
    }
}
