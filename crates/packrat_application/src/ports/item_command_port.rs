use async_trait::async_trait;
use packrat_domain::item::{Entity, EntityId, EntityName};

#[async_trait]
pub trait ItemCommandPort: Send + Sync {
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity;
}
