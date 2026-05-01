use packrat_domain::entity::Entity;

use crate::ports::{ItemQueryPort, ItemSearchQuery};

pub async fn execute(port: &impl ItemQueryPort, query: &ItemSearchQuery) -> Vec<Entity> {
    port.search_items(query).await
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
        async fn get_item_by_id(
            &self,
            _id: packrat_domain::entity::EntityId,
        ) -> Option<Entity> {
            None
        }

        async fn list_active_items(&self) -> Vec<Entity> {
            self.0.clone()
        }

        async fn search_items(&self, query: &ItemSearchQuery) -> Vec<Entity> {
            self.0
                .iter()
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
                .cloned()
                .collect()
        }

        async fn list_child_items(
            &self,
            parent_id: packrat_domain::entity::EntityId,
        ) -> Vec<Entity> {
            self.0
                .iter()
                .filter(|e| e.parent == Some(parent_id))
                .cloned()
                .collect()
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
    async fn filters_by_exact_name() {
        let port = MockPort(vec![entity(1, "Alpha"), entity(2, "Beta")]);
        let q = ItemSearchQuery {
            name: Some("Beta".into()),
            fuzzyname: None,
        };
        assert_eq!(execute(&port, &q).await, vec![entity(2, "Beta")]);
    }

    #[tokio::test]
    async fn filters_by_fuzzy_substring() {
        let port = MockPort(vec![entity(1, "Canon R5"), entity(2, "Nikon Z9")]);
        let q = ItemSearchQuery {
            name: None,
            fuzzyname: Some("nik".into()),
        };
        assert_eq!(execute(&port, &q).await, vec![entity(2, "Nikon Z9")]);
    }

    #[tokio::test]
    async fn combines_name_and_fuzzyname_with_and() {
        let port = MockPort(vec![
            entity(1, "Toolbox"),
            entity(2, "Red Toolbox"),
            entity(3, "Red Bucket"),
        ]);
        let q = ItemSearchQuery {
            name: Some("Red Toolbox".into()),
            fuzzyname: Some("tool".into()),
        };
        assert_eq!(execute(&port, &q).await, vec![entity(2, "Red Toolbox")]);
    }
}
