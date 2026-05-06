use async_trait::async_trait;
use packrat_domain::{
    aggregates::partial_asset::PartialAsset,
    asset::{Asset, AssetId, AssetName},
};

#[async_trait]
pub trait AssetCommandPort: Send + Sync {
    async fn create_asset(&self, name: AssetName, parent: Option<AssetId>) -> Asset;
    async fn update_asset(&self, id: AssetId, changes: PartialAsset) -> Result<(), String>;
    async fn delete_asset(&self, id: AssetId) -> Result<(), String>;
}
