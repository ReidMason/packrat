use packrat_domain::entity::EntityId;

use crate::ports::ItemCommandPort;

pub async fn execute(port: &impl ItemCommandPort, id: EntityId) -> Result<(), String> {
    port.delete_entity(id).await
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
        async fn delete_entity(&self, id: EntityId) -> Result<(), String> {
            if id == EntityId::from(99) {
                Ok(())
            } else {
                Err("Entity not found".to_string())
            }
        }
    }

    #[tokio::test]
    async fn execute_deletes_entity_via_port() {
        let port = MockItemCommand;
        let id_to_delete = EntityId::from(99);

        let result = execute(&port, id_to_delete).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn execute_nothing_to_delete_via_port() {
        let port = MockItemCommand;
        let id_to_delete = EntityId::from(404);

        let result = execute(&port, id_to_delete).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
