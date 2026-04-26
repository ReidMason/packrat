use async_trait::async_trait;
use packrat_domain::entity::{Entity, EntityId};

#[async_trait]
pub trait ItemQueryPort: Send + Sync {
    async fn get_item_by_id(&self, id: EntityId) -> Option<Entity>;
}
