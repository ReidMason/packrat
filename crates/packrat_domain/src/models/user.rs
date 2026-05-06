use argon2::{PasswordHasher, PasswordVerifier};

use super::entity::EntityTimestamp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn verify(&self, plain_password: &str) -> bool {
        if let Ok(parsed_hash) = argon2::PasswordHash::new(&self.0) {
            return argon2::Argon2::default()
                .verify_password(plain_password.as_bytes(), &parsed_hash)
                .is_ok();
        }
        false
    }

    pub fn generate(plain_password: &str) -> Result<Self, String> {
        let salt = argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        );

        let hashed_string = argon2::Argon2::default()
            .hash_password(plain_password.as_bytes(), &salt)
            .map_err(|err| format!("Password hashing failed: {}", err))?
            .to_string();

        Ok(Self(hashed_string))
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        PasswordHash(value)
    }
}

impl From<PasswordHash> for String {
    fn from(value: PasswordHash) -> Self {
        value.0
    }
}

impl From<&str> for PasswordHash {
    fn from(value: &str) -> Self {
        PasswordHash(value.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct UserId(i64);

impl From<i64> for UserId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<UserId> for i64 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_string())
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password: PasswordHash,
    pub created: EntityTimestamp,
    pub updated: EntityTimestamp,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        password: PasswordHash,
        created: EntityTimestamp,
        updated: EntityTimestamp,
    ) -> Self {
        Self {
            id,
            email,
            password,
            created,
            updated,
        }
    }
}
