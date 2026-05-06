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

