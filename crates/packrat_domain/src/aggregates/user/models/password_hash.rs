use argon2::{PasswordHasher, PasswordVerifier};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Creates a hash from a given string
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

    pub fn verify(&self, plain_password: &str) -> bool {
        if let Ok(parsed_hash) = argon2::PasswordHash::new(&self.0) {
            return argon2::Argon2::default()
                .verify_password(plain_password.as_bytes(), &parsed_hash)
                .is_ok();
        }
        false
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<PasswordHash> for String {
    fn from(value: PasswordHash) -> Self {
        value.0
    }
}
