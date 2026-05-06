pub mod models;

pub use models::{tenant_id::TenantId, tenant_name::TenantName};

use crate::asset::AssetTimestamp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
    pub created: AssetTimestamp,
    pub updated: AssetTimestamp,
}

impl Tenant {
    pub fn new(
        id: TenantId,
        name: TenantName,
        created: AssetTimestamp,
        updated: AssetTimestamp,
    ) -> Self {
        Self {
            id,
            name,
            created,
            updated,
        }
    }
}
