use super::entity::EntityTimestamp;

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
    pub created: EntityTimestamp,
    pub updated: EntityTimestamp,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        created: EntityTimestamp,
        updated: EntityTimestamp,
    ) -> Self {
        Self {
            id,
            email,
            created,
            updated,
        }
    }
}
