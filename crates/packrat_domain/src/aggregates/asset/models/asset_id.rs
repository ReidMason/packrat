#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AssetId(i64);

impl From<i64> for AssetId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<AssetId> for i64 {
    fn from(id: AssetId) -> Self {
        id.0
    }
}
