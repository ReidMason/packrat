pub mod models;

pub use models::{asset_id::AssetId, asset_name::AssetName, asset_timestamp::AssetTimestamp};

use crate::aggregates::partial_asset::PartialAsset;
use crate::aggregates::tenant::TenantId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Asset {
    pub id: AssetId,
    pub tenant_id: TenantId,
    pub name: AssetName,
    pub parent: Option<AssetId>,
    pub created: AssetTimestamp,
    pub deleted: Option<AssetTimestamp>,
}

impl Asset {
    pub fn new(
        id: AssetId,
        tenant_id: TenantId,
        name: AssetName,
        parent: Option<AssetId>,
        created: AssetTimestamp,
        deleted: Option<AssetTimestamp>,
    ) -> Self {
        Self {
            id,
            tenant_id,
            name,
            parent,
            created,
            deleted,
        }
    }

    pub fn apply_partial(&mut self, changes: PartialAsset) {
        if let Some(name) = changes.name {
            self.name = name;
        }
        if let Some(parent) = changes.parent {
            self.parent = parent;
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted.is_some()
    }

    pub fn mark_as_deleted(&mut self) {
        if self.deleted.is_none() {
            self.deleted = Some(AssetTimestamp::now());
        }
    }
}
