use packrat_domain::asset::AssetId;
use packrat_domain::aggregates::partial_asset::PartialAsset;
use packrat_domain::tenant::TenantId;

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    tenant_id: TenantId,
    id: AssetId,
    changes: PartialAsset,
) -> Result<(), String> {
    port.update_asset(tenant_id, id, changes).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::asset::{Asset, AssetName};
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

        async fn delete_asset(&self, _tenant_id: TenantId, _id: AssetId) -> Result<(), String> {
            unimplemented!()
        }

        async fn update_asset(
            &self,
            _tenant_id: TenantId,
            id: AssetId,
            _changes: PartialAsset,
        ) -> Result<(), String> {
            if id == AssetId::from(1) {
                Ok(())
            } else {
                Err("Entity not found".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_update_success() {
        let port = MockAssetCommand;
        let id = AssetId::from(1);
        let changes = PartialAsset {
            name: Some(AssetName::from("New Name")),
            parent: None,
        };

        let result = execute(&port, TenantId::from(1), id, changes).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let port = MockAssetCommand;
        let id = AssetId::from(404);
        let changes = PartialAsset::default();

        let result = execute(&port, TenantId::from(1), id, changes).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
