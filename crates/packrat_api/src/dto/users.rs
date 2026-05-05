use packrat_domain::user::User;
use serde::Serialize;

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    pub email: String,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: i64,
    pub email: String,
    pub created: String,
    pub updated: String,
}

impl UserDto {
    pub fn from_user(user: User) -> Self {
        Self {
            id: i64::from(user.id),
            email: user.email.as_str().to_string(),
            created: user.created.to_string(),
            updated: user.updated.to_string(),
        }
    }
}
