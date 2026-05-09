use packrat_domain::asset::Asset;
use packrat_domain::tenant::TenantId;

use crate::ports::{AssetQueryPort, AssetSearchQuery};

pub async fn execute(
    port: &impl AssetQueryPort,
    tenant_id: TenantId,
    query: &AssetSearchQuery,
) -> Vec<Asset> {
    port.search_assets(tenant_id, query).await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::asset::{AssetId, AssetName, AssetTimestamp};
    use packrat_domain::tenant::TenantId;

    use super::*;
    use crate::ports::AssetQueryPort;

    struct MockPort(Vec<Asset>);

    #[async_trait]
    impl AssetQueryPort for MockPort {
        async fn get_asset_by_id(&self, _tenant_id: TenantId, _id: AssetId) -> Option<Asset> {
            None
        }

        async fn list_active_assets(&self, _tenant_id: TenantId) -> Vec<Asset> {
            self.0.clone()
        }

        async fn search_assets(&self, _tenant_id: TenantId, query: &AssetSearchQuery) -> Vec<Asset> {
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
                        .map(|n| e.name.as_str().to_lowercase().contains(&n.to_lowercase()))
                        .unwrap_or(true);
                    name_ok && fuzzy_ok
                })
                .cloned()
                .collect()
        }

        async fn list_child_assets(
            &self,
            _tenant_id: TenantId,
            parent_id: AssetId,
        ) -> Vec<Asset> {
            self.0
                .iter()
                .filter(|e| e.parent == Some(parent_id))
                .cloned()
                .collect()
        }
    }

    fn entity(id: i64, name: &str) -> Asset {
        Asset::new(
            id.into(),
            TenantId::from(1),
            AssetName::from(name),
            None,
            AssetTimestamp::static_for_tests(),
            None,
        )
    }

    #[tokio::test]
    async fn filters_by_exact_name() {
        let port = MockPort(vec![entity(1, "Alpha"), entity(2, "Beta")]);
        let q = AssetSearchQuery {
            name: Some("Beta".into()),
            fuzzyname: None,
        };
        assert_eq!(
            execute(&port, TenantId::from(1), &q).await,
            vec![entity(2, "Beta")]
        );
    }

    #[tokio::test]
    async fn filters_by_fuzzy_substring() {
        let port = MockPort(vec![entity(1, "Canon R5"), entity(2, "Nikon Z9")]);
        let q = AssetSearchQuery {
            name: None,
            fuzzyname: Some("nik".into()),
        };
        assert_eq!(
            execute(&port, TenantId::from(1), &q).await,
            vec![entity(2, "Nikon Z9")]
        );
    }

    #[tokio::test]
    async fn combines_name_and_fuzzyname_with_and() {
        let port = MockPort(vec![
            entity(1, "Toolbox"),
            entity(2, "Red Toolbox"),
            entity(3, "Red Bucket"),
        ]);
        let q = AssetSearchQuery {
            name: Some("Red Toolbox".into()),
            fuzzyname: Some("tool".into()),
        };
        assert_eq!(
            execute(&port, TenantId::from(1), &q).await,
            vec![entity(2, "Red Toolbox")]
        );
    }
}
