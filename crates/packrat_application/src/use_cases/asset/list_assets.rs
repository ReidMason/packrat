use packrat_domain::entity::Entity;

use crate::ports::AssetQueryPort;

pub async fn execute(port: &impl AssetQueryPort) -> Vec<Entity> {
    port.list_active_assets().await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::entity::{EntityName, EntityTimestamp};

    use super::*;
    use crate::ports::AssetQueryPort;

    struct MockPort(Vec<Entity>);

    #[async_trait]
    impl AssetQueryPort for MockPort {
        async fn get_asset_by_id(&self, _id: packrat_domain::entity::EntityId) -> Option<Entity> {
            None
        }

        async fn list_active_assets(&self) -> Vec<Entity> {
            self.0.clone()
        }

        async fn search_assets(
            &self,
            _query: &crate::ports::AssetSearchQuery,
        ) -> Vec<Entity> {
            Vec::new()
        }

        async fn list_child_assets(
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
