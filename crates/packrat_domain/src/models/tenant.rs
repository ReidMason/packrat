use super::entity::EntityTimestamp;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TenantName(String);

impl TenantName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for TenantName {
    fn from(s: &str) -> Self {
        TenantName(s.to_string())
    }
}

impl From<String> for TenantName {
    fn from(s: String) -> Self {
        TenantName(s)
    }
}

impl From<TenantName> for String {
    fn from(name: TenantName) -> Self {
        name.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
    pub created: EntityTimestamp,
    pub updated: EntityTimestamp,
}

impl Tenant {
    pub fn new(
        id: TenantId,
        name: TenantName,
        created: EntityTimestamp,
        updated: EntityTimestamp,
    ) -> Self {
        Self {
            id,
            name,
            created,
            updated,
        }
    }
}
