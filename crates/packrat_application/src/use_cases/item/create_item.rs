use packrat_domain::item::{Entity, EntityId, EntityName};

use crate::ports::ItemCommandPort;

pub async fn execute(port: &impl ItemCommandPort, name: EntityName, parent: Option<EntityId>) -> Entity {
    port.create_item(name, parent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::item::EntityId;

    struct MockItemCommand;

    #[async_trait]
    impl ItemCommandPort for MockItemCommand {
        async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
            Entity::new(EntityId::from(99), name, parent)
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
