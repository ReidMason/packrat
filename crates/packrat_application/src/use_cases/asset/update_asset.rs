use packrat_domain::entity::EntityId;
use packrat_domain::models::partial_entity::PartialEntity;

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    id: EntityId,
    changes: PartialEntity,
) -> Result<(), String> {
    port.update_asset(id, changes).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::entity::{Entity, EntityId, EntityName};

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, _name: EntityName, _parent: Option<EntityId>) -> Entity {
            unimplemented!()
        }

        async fn delete_asset(&self, _id: EntityId) -> Result<(), String> {
            unimplemented!()
        }

        async fn update_asset(&self, id: EntityId, _changes: PartialEntity) -> Result<(), String> {
            if id == EntityId::from(1) {
                Ok(())
            } else {
                Err("Entity not found".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_update_success() {
        let port = MockAssetCommand;
        let id = EntityId::from(1);
        let changes = PartialEntity {
            name: Some(EntityName::from("New Name")),
            parent: None,
        };

        let result = execute(&port, id, changes).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let port = MockAssetCommand;
        let id = EntityId::from(404);
        let changes = PartialEntity::default();

        let result = execute(&port, id, changes).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
