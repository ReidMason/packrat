use packrat_domain::entity::{Entity, EntityId};

use crate::ports::AssetQueryPort;

pub async fn execute(port: &impl AssetQueryPort, parent_id: EntityId) -> Vec<Entity> {
    port.list_child_assets(parent_id).await
}
