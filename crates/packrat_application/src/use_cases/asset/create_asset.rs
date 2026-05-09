use packrat_domain::asset::{Asset, AssetId, AssetName};
use packrat_domain::tenant::TenantId;

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    tenant_id: TenantId,
    name: AssetName,
    parent: Option<AssetId>,
) -> Result<Asset, String> {
    port.create_asset(tenant_id, name, parent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{aggregates::partial_asset::PartialAsset, asset::AssetTimestamp};

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(
            &self,
            tenant_id: TenantId,
            name: AssetName,
            parent: Option<AssetId>,
        ) -> Result<Asset, String> {
            let created = AssetTimestamp::now();
            let deleted = None;
            Ok(Asset::new(
                AssetId::from(99),
                tenant_id,
                name,
                parent,
                created,
                deleted,
            ))
        }
        async fn update_asset(
            &self,
            _tenant_id: TenantId,
            _id: AssetId,
            _changes: PartialAsset,
        ) -> Result<(), String> {
            unimplemented!()
        }
        async fn delete_asset(&self, _tenant_id: TenantId, _id: AssetId) -> Result<(), String> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn execute_creates_asset_via_port() {
        let port = MockAssetCommand;
        let parent = Some(AssetId::from(1));
        let tid = TenantId::from(7);
        let asset = execute(&port, tid, AssetName::from("alpha"), parent)
            .await
            .unwrap();
        assert_eq!(asset.id, AssetId::from(99));
        assert_eq!(asset.tenant_id, tid);
        assert_eq!(asset.name, AssetName::from("alpha"));
        assert_eq!(asset.parent, parent);
    }

    #[tokio::test]
    async fn execute_creates_root_asset() {
        let port = MockAssetCommand;
        let tid = TenantId::from(1);
        let asset = execute(&port, tid, AssetName::from("root"), None)
            .await
            .unwrap();
        assert_eq!(asset.parent, None);
    }
}
