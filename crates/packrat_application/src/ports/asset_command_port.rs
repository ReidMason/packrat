use async_trait::async_trait;
use packrat_domain::{
    aggregates::partial_asset::PartialAsset,
    asset::{Asset, AssetId, AssetName},
    tenant::TenantId,
};

#[async_trait]
pub trait AssetCommandPort: Send + Sync {
    async fn create_asset(
        &self,
        tenant_id: TenantId,
        name: AssetName,
        parent: Option<AssetId>,
    ) -> Result<Asset, String>;

    async fn update_asset(
        &self,
        tenant_id: TenantId,
        id: AssetId,
        changes: PartialAsset,
    ) -> Result<(), String>;

    async fn delete_asset(&self, tenant_id: TenantId, id: AssetId) -> Result<(), String>;
}
