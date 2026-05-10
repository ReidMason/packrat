#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TenantId(i64);

impl From<i64> for TenantId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<TenantId> for i64 {
    fn from(id: TenantId) -> Self {
        id.0
    }
}
