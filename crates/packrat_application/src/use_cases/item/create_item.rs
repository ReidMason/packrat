use packrat_domain::entity::{Entity, EntityId, EntityName};

use crate::ports::ItemCommandPort;

pub async fn execute(
    port: &impl ItemCommandPort,
    name: EntityName,
    parent: Option<EntityId>,
) -> Entity {
    port.create_item(name, parent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{entity::{EntityId, EntityTimestamp}, models::partial_entity::PartialEntity};

    struct MockItemCommand;

    #[async_trait]
    impl ItemCommandPort for MockItemCommand {
        async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
            let created = EntityTimestamp::now();
            let deleted = None;
            Entity::new(EntityId::from(99), name, parent, created, deleted)
        }
        async fn delete_entity(&self, _id: EntityId) -> Result<(), String> {
            unimplemented!()
        }
        async fn update_entity(&self, _id: EntityId, _changes: PartialEntity) -> Result<(), String> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn execute_creates_item_via_port() {
        let port = MockItemCommand;
        let parent = Some(EntityId::from(1));
        let item = execute(&port, EntityName::from("alpha"), parent).await;
        assert_eq!(item.id, EntityId::from(99));
        assert_eq!(item.name, EntityName::from("alpha"));
        assert_eq!(item.parent, parent);
    }

    #[tokio::test]
    async fn execute_creates_root_item() {
        let port = MockItemCommand;
        let item = execute(&port, EntityName::from("root"), None).await;
        assert_eq!(item.parent, None);
    }
}
