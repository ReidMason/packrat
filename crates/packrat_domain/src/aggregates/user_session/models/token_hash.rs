#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenHash(Vec<u8>);

impl TokenHash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for TokenHash {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<TokenHash> for Vec<u8> {
    fn from(value: TokenHash) -> Self {
        value.0
    }
}
