use packrat_domain::asset::Asset;
use packrat_domain::tenant::TenantId;

use crate::ports::AssetQueryPort;

pub async fn execute(port: &impl AssetQueryPort, tenant_id: TenantId) -> Vec<Asset> {
    port.list_active_assets(tenant_id).await
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

        async fn search_assets(
            &self,
            _tenant_id: TenantId,
            _query: &crate::ports::AssetSearchQuery,
        ) -> Vec<Asset> {
            Vec::new()
        }

        async fn list_child_assets(
            &self,
            _tenant_id: TenantId,
            _parent_id: AssetId,
        ) -> Vec<Asset> {
            Vec::new()
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
    async fn forwards_port_list() {
        let expected = vec![entity(1, "a"), entity(2, "b")];
        let port = MockPort(expected.clone());
        assert_eq!(execute(&port, TenantId::from(1)).await, expected);
    }
}
