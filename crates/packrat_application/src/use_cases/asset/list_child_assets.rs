use packrat_domain::asset::{Asset, AssetId};
use packrat_domain::tenant::TenantId;

use crate::ports::AssetQueryPort;

pub async fn execute(
    port: &impl AssetQueryPort,
    tenant_id: TenantId,
    parent_id: AssetId,
) -> Vec<Asset> {
    port.list_child_assets(tenant_id, parent_id).await
}
