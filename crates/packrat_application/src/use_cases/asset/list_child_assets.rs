use packrat_domain::asset::AssetId;
use packrat_domain::tenant::TenantId;

use crate::asset_with_tags::AssetWithTags;
use crate::ports::AssetQueryPort;

pub async fn execute(
    port: &impl AssetQueryPort,
    tenant_id: TenantId,
    parent_id: AssetId,
) -> Vec<AssetWithTags> {
    port.list_child_assets(tenant_id, parent_id).await
}
