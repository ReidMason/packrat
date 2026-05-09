use packrat_domain::asset::AssetId;
use packrat_domain::tag::TagId;
use packrat_domain::tenant::TenantId;

use crate::ports::TagCommandPort;

pub async fn execute(
    port: &impl TagCommandPort,
    tenant_id: TenantId,
    asset_id: AssetId,
    tag_ids: Vec<TagId>,
) -> Result<(), String> {
    port.set_asset_tags(tenant_id, asset_id, tag_ids).await
}
