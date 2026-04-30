use packrat_domain::entity::{Entity, EntityId};

use crate::ports::ItemQueryPort;

pub async fn execute(port: &impl ItemQueryPort, id: EntityId) -> Option<Entity> {
    port.get_item_by_id(id).await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::entity::{EntityName, EntityTimestamp};

    use super::*;

    struct MockItemQuery;

    fn test_timestamp() -> EntityTimestamp {
        chrono::DateTime::from_timestamp(1735689600, 0)
            .unwrap()
            .into()
    }

    fn stub_item(id: EntityId) -> Entity {
        Entity::new(
            id,
            EntityName::from("from infrastructure stub"),
            Some(EntityId::from(1)),
            test_timestamp(),
            None,
        )
    }

    #[async_trait]
    impl ItemQueryPort for MockItemQuery {
        async fn get_item_by_id(&self, id: EntityId) -> Option<Entity> {
            if id == EntityId::from(1) {
                Some(stub_item(id))
            } else {
                None
            }
        }
    }

    #[tokio::test]
    async fn execute_returns_item_when_present() {
        let port = MockItemQuery;
        assert_eq!(
            execute(&port, EntityId::from(1)).await,
            Some(stub_item(EntityId::from(1)))
        );
    }

    #[tokio::test]
    async fn execute_returns_none_when_missing() {
        let port = MockItemQuery;
        assert_eq!(execute(&port, EntityId::from(999)).await, None);
    }
}
