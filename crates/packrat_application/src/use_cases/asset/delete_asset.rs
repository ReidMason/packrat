use packrat_domain::entity::EntityId;

use crate::ports::AssetCommandPort;

pub async fn execute(port: &impl AssetCommandPort, id: EntityId) -> Result<(), String> {
    port.delete_asset(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::entity::{Entity, EntityId, EntityName};
    use packrat_domain::models::partial_entity::PartialEntity;

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, _name: EntityName, _parent: Option<EntityId>) -> Entity {
            unimplemented!()
        }

        async fn update_asset(&self, _id: EntityId, _changes: PartialEntity) -> Result<(), String> {
            unimplemented!()
        }

        async fn delete_asset(&self, id: EntityId) -> Result<(), String> {
            if id == EntityId::from(1) {
                Ok(())
            } else {
                Err("not found".into())
            }
        }
    }

    #[tokio::test]
    async fn delete_ok() {
        let port = MockAssetCommand;
        assert!(execute(&port, EntityId::from(1)).await.is_ok());
    }

    #[tokio::test]
    async fn delete_missing() {
        let port = MockAssetCommand;
        assert!(execute(&port, EntityId::from(999)).await.is_err());
    }
}
