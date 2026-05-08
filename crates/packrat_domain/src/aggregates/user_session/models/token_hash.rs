use rand::Rng;
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenHash(Vec<u8>);

impl TokenHash {
    pub fn generate() -> (Self, String) {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);

        let raw_hex = hex::encode(bytes);

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash_result = hasher.finalize().to_vec();

        (Self(hash_result), raw_hex)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_sha256_digest(bytes: Vec<u8>) -> Option<Self> {
        if bytes.len() != 32 {
            return None;
        }
        Some(Self(bytes))
    }

    pub fn from_login_token_hex(token: &str) -> Option<Self> {
        let bytes = hex::decode(token.trim()).ok()?;
        if bytes.len() != 32 {
            return None;
        }
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Self::from_sha256_digest(hasher.finalize().to_vec())
    }
}

impl From<TokenHash> for Vec<u8> {
    fn from(value: TokenHash) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::TokenHash;

    #[test]
    fn from_login_token_hex_matches_generate() {
        let (stored, hex) = TokenHash::generate();
        let decoded = TokenHash::from_login_token_hex(&hex).expect("valid hex");
        assert_eq!(decoded, stored);
    }
}
