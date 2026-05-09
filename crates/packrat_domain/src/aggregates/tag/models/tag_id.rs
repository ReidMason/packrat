#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct TagId(i64);

impl From<i64> for TagId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<TagId> for i64 {
    fn from(id: TagId) -> Self {
        id.0
    }
}
