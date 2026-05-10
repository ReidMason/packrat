use async_trait::async_trait;
use packrat_domain::asset::AssetId;
use packrat_domain::tag::{Tag, TagId, TagName};
use packrat_domain::tenant::TenantId;

#[async_trait]
pub trait TagCommandPort: Send + Sync {
    async fn ensure_tag(&self, tenant_id: TenantId, name: TagName) -> Result<Tag, String>;

    /// Replaces all tags on the asset with the given set (empty clears tags).
    async fn set_asset_tags(
        &self,
        tenant_id: TenantId,
        asset_id: AssetId,
        tag_ids: Vec<TagId>,
    ) -> Result<(), String>;
}
