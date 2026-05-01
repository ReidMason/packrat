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
        EntityTimestamp::static_for_tests()
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

        async fn list_active_items(&self) -> Vec<Entity> {
            vec![stub_item(EntityId::from(1))]
        }

        async fn search_items(&self, query: &crate::ports::ItemSearchQuery) -> Vec<Entity> {
            self.list_active_items()
                .await
                .into_iter()
                .filter(|e| {
                    let name_ok = query
                        .name
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|n| e.name.as_str() == n)
                        .unwrap_or(true);
                    let fuzzy_ok = query
                        .fuzzyname
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|n| {
                            e.name
                                .as_str()
                                .to_lowercase()
                                .contains(&n.to_lowercase())
                        })
                        .unwrap_or(true);
                    name_ok && fuzzy_ok
                })
                .collect()
        }

        async fn list_child_items(
            &self,
            parent_id: EntityId,
        ) -> Vec<Entity> {
            self.list_active_items()
                .await
                .into_iter()
                .filter(|e| e.parent == Some(parent_id))
                .collect()
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
