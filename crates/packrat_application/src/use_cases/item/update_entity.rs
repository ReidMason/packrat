use packrat_domain::entity::EntityId;
use packrat_domain::models::partial_entity::PartialEntity;

use crate::ports::ItemCommandPort;

pub async fn execute(
    port: &impl ItemCommandPort,
    id: EntityId,
    changes: PartialEntity,
) -> Result<(), String> {
    port.update_entity(id, changes).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::entity::{Entity, EntityId, EntityName};

    struct MockItemCommand;

    #[async_trait]
    impl ItemCommandPort for MockItemCommand {
        async fn create_item(&self, _name: EntityName, _parent: Option<EntityId>) -> Entity {
            unimplemented!()
        }

        async fn delete_entity(&self, _id: EntityId) -> Result<(), String> {
            unimplemented!()
        }

        async fn update_entity(&self, id: EntityId, _changes: PartialEntity) -> Result<(), String> {
            if id == EntityId::from(1) {
                Ok(())
            } else {
                Err("Entity not found".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_update_success() {
        let port = MockItemCommand;
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
        let port = MockItemCommand;
        let id = EntityId::from(404);
        let changes = PartialEntity::default();

        let result = execute(&port, id, changes).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
