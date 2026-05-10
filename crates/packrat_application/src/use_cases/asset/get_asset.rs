use packrat_domain::asset::AssetId;
use packrat_domain::tenant::TenantId;

use crate::asset_with_tags::AssetWithTags;
use crate::ports::AssetQueryPort;

pub async fn execute(
    port: &impl AssetQueryPort,
    tenant_id: TenantId,
    id: AssetId,
) -> Option<AssetWithTags> {
    port.get_asset_by_id(tenant_id, id).await
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use packrat_domain::asset::{Asset, AssetId, AssetName, AssetTimestamp};
    use packrat_domain::tenant::TenantId;

    use super::*;
    use crate::ports::{AssetQueryPort, AssetSearchQuery};

    struct MockAssetQuery;

    fn test_timestamp() -> AssetTimestamp {
        AssetTimestamp::static_for_tests()
    }

    fn stub_entity(id: AssetId) -> Asset {
        Asset::new(
            id,
            TenantId::from(1),
            AssetName::from("from infrastructure stub"),
            Some(AssetId::from(1)),
            test_timestamp(),
            None,
        )
    }

    #[async_trait]
    impl AssetQueryPort for MockAssetQuery {
        async fn get_asset_by_id(
            &self,
            _tenant_id: TenantId,
            id: AssetId,
        ) -> Option<AssetWithTags> {
            if id == AssetId::from(1) {
                Some(AssetWithTags::new(stub_entity(id), vec![]))
            } else {
                None
            }
        }

        async fn list_active_assets(&self, _tenant_id: TenantId) -> Vec<AssetWithTags> {
            vec![AssetWithTags::new(stub_entity(AssetId::from(1)), vec![])]
        }

        async fn search_assets(
            &self,
            tenant_id: TenantId,
            query: &AssetSearchQuery,
        ) -> Vec<AssetWithTags> {
            self.list_active_assets(tenant_id)
                .await
                .into_iter()
                .filter(|e| {
                    let name_ok = query
                        .name
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|n| e.asset.name.as_str() == n)
                        .unwrap_or(true);
                    let fuzzy_ok = query
                        .fuzzyname
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|n| {
                            e.asset
                                .name
                                .as_str()
                                .to_lowercase()
                                .contains(&n.to_lowercase())
                        })
                        .unwrap_or(true);
                    name_ok && fuzzy_ok
                })
                .collect()
        }

        async fn list_child_assets(
            &self,
            tenant_id: TenantId,
            parent_id: AssetId,
        ) -> Vec<AssetWithTags> {
            self.list_active_assets(tenant_id)
                .await
                .into_iter()
                .filter(|e| e.asset.parent == Some(parent_id))
                .collect()
        }
    }

    #[tokio::test]
    async fn execute_returns_asset_when_present() {
        let port = MockAssetQuery;
        assert_eq!(
            execute(&port, TenantId::from(1), AssetId::from(1)).await,
            Some(AssetWithTags::new(stub_entity(AssetId::from(1)), vec![]))
        );
    }

    #[tokio::test]
    async fn execute_returns_none_when_missing() {
        let port = MockAssetQuery;
        assert_eq!(
            execute(&port, TenantId::from(1), AssetId::from(999)).await,
            None
        );
    }
}
