pub mod models;

pub use models::{TagId, TagName};

use crate::aggregates::tenant::TenantId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub id: TagId,
    pub tenant_id: TenantId,
    pub name: TagName,
}

impl Tag {
    pub fn new(id: TagId, tenant_id: TenantId, name: TagName) -> Self {
        Self {
            id,
            tenant_id,
            name,
        }
    }
}
