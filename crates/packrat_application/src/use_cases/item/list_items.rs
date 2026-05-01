use packrat_domain::entity::Entity;

use crate::ports::ItemQueryPort;

pub async fn execute(port: &impl ItemQueryPort) -> Vec<Entity> {
    port.list_active_items().await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::entity::{EntityName, EntityTimestamp};

    use super::*;
    use crate::ports::ItemQueryPort;

    struct MockPort(Vec<Entity>);

    #[async_trait]
    impl ItemQueryPort for MockPort {
        async fn get_item_by_id(&self, _id: packrat_domain::entity::EntityId) -> Option<Entity> {
            None
        }

        async fn list_active_items(&self) -> Vec<Entity> {
            self.0.clone()
        }

        async fn search_items(
            &self,
            _query: &crate::ports::ItemSearchQuery,
        ) -> Vec<Entity> {
            Vec::new()
        }

        async fn list_child_items(
            &self,
            _parent_id: packrat_domain::entity::EntityId,
        ) -> Vec<Entity> {
            Vec::new()
        }
    }

    fn entity(id: i64, name: &str) -> Entity {
        Entity::new(
            id.into(),
            EntityName::from(name),
            None,
            EntityTimestamp::static_for_tests(),
            None,
        )
    }

    #[tokio::test]
    async fn forwards_port_list() {
        let expected = vec![entity(1, "a"), entity(2, "b")];
        let port = MockPort(expected.clone());
        assert_eq!(execute(&port).await, expected);
    }
}
