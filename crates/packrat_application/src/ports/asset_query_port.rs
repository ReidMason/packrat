use async_trait::async_trait;
use packrat_domain::asset::AssetId;
use packrat_domain::tenant::TenantId;

use super::asset_search_query::AssetSearchQuery;
use crate::asset_with_tags::AssetWithTags;

#[async_trait]
pub trait AssetQueryPort: Send + Sync {
    async fn get_asset_by_id(&self, tenant_id: TenantId, id: AssetId) -> Option<AssetWithTags>;

    async fn list_active_assets(&self, tenant_id: TenantId) -> Vec<AssetWithTags>;

    async fn search_assets(
        &self,
        tenant_id: TenantId,
        query: &AssetSearchQuery,
    ) -> Vec<AssetWithTags>;

    async fn list_child_assets(
        &self,
        tenant_id: TenantId,
        parent_id: AssetId,
    ) -> Vec<AssetWithTags>;
}
