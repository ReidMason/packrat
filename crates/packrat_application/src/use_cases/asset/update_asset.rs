use packrat_domain::asset::AssetId;
use packrat_domain::aggregates::partial_asset::PartialAsset;

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    id: AssetId,
    changes: PartialAsset,
) -> Result<(), String> {
    port.update_asset(id, changes).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::asset::{Asset, AssetName};

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, _name: AssetName, _parent: Option<AssetId>) -> Asset {
            unimplemented!()
        }

        async fn delete_asset(&self, _id: AssetId) -> Result<(), String> {
            unimplemented!()
        }

        async fn update_asset(&self, id: AssetId, _changes: PartialAsset) -> Result<(), String> {
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

        let result = execute(&port, id, changes).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let port = MockAssetCommand;
        let id = AssetId::from(404);
        let changes = PartialAsset::default();

        let result = execute(&port, id, changes).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Entity not found");
    }
}
