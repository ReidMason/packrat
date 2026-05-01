use packrat_domain::entity::{Entity, EntityId};

use crate::ports::ItemQueryPort;

pub async fn execute(port: &impl ItemQueryPort, parent_id: EntityId) -> Vec<Entity> {
    port.list_child_items(parent_id).await
}
