use async_trait::async_trait;
use packrat_domain::entity::{Entity, EntityId, EntityName};

#[async_trait]
pub trait ItemCommandPort: Send + Sync {
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity;
    async fn delete_entity(&self, id: EntityId) -> Result<(), String>;
}
