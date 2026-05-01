use async_trait::async_trait;
use packrat_domain::{
    entity::{Entity, EntityId, EntityName},
    models::partial_entity::PartialEntity,
};

#[async_trait]
pub trait AssetCommandPort: Send + Sync {
    async fn create_asset(&self, name: EntityName, parent: Option<EntityId>) -> Entity;
    async fn update_asset(&self, id: EntityId, changes: PartialEntity) -> Result<(), String>;
    async fn delete_asset(&self, id: EntityId) -> Result<(), String>;
}
