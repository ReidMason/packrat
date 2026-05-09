use packrat_domain::asset::AssetId;
use packrat_domain::tenant::TenantId;

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    tenant_id: TenantId,
    id: AssetId,
) -> Result<(), String> {
    port.delete_asset(tenant_id, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{
        aggregates::partial_asset::PartialAsset,
        asset::{Asset, AssetName},
    };
    use packrat_domain::tenant::TenantId;

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(
            &self,
            _tenant_id: TenantId,
            _name: AssetName,
            _parent: Option<AssetId>,
        ) -> Result<Asset, String> {
            unimplemented!()
        }

        async fn update_asset(
            &self,
            _tenant_id: TenantId,
            _id: AssetId,
            _changes: PartialAsset,
        ) -> Result<(), String> {
            unimplemented!()
        }

        async fn delete_asset(&self, _tenant_id: TenantId, id: AssetId) -> Result<(), String> {
            if id == AssetId::from(1) {
                Ok(())
            } else {
                Err("not found".into())
            }
        }
    }

    #[tokio::test]
    async fn delete_ok() {
        let port = MockAssetCommand;
        assert!(execute(&port, TenantId::from(1), AssetId::from(1))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn delete_missing() {
        let port = MockAssetCommand;
        assert!(execute(&port, TenantId::from(1), AssetId::from(999))
            .await
            .is_err());
    }
}
