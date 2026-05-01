use packrat_domain::entity::{Entity, EntityId, EntityName};

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    name: EntityName,
    parent: Option<EntityId>,
) -> Entity {
    port.create_asset(name, parent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{
        entity::{EntityId, EntityTimestamp},
        models::partial_entity::PartialEntity,
    };

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
            let created = EntityTimestamp::now();
            let deleted = None;
            Entity::new(EntityId::from(99), name, parent, created, deleted)
        }
        async fn update_asset(&self, _id: EntityId, _changes: PartialEntity) -> Result<(), String> {
            unimplemented!()
        }
        async fn delete_asset(&self, _id: EntityId) -> Result<(), String> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn execute_creates_asset_via_port() {
        let port = MockAssetCommand;
        let parent = Some(EntityId::from(1));
        let asset = execute(&port, EntityName::from("alpha"), parent).await;
        assert_eq!(asset.id, EntityId::from(99));
        assert_eq!(asset.name, EntityName::from("alpha"));
        assert_eq!(asset.parent, parent);
    }

    #[tokio::test]
    async fn execute_creates_root_asset() {
        let port = MockAssetCommand;
        let asset = execute(&port, EntityName::from("root"), None).await;
        assert_eq!(asset.parent, None);
    }
}
