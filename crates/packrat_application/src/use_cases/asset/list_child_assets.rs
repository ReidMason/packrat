use packrat_domain::asset::{Asset, AssetId};

use crate::ports::AssetQueryPort;

pub async fn execute(port: &impl AssetQueryPort, parent_id: AssetId) -> Vec<Asset> {
    port.list_child_assets(parent_id).await
}
