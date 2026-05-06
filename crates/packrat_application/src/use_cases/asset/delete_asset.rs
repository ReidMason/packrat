use packrat_domain::asset::AssetId;

use crate::ports::AssetCommandPort;

pub async fn execute(port: &impl AssetCommandPort, id: AssetId) -> Result<(), String> {
    port.delete_asset(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{
        aggregates::partial_asset::PartialAsset,
        asset::{Asset, AssetName},
    };

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, _name: AssetName, _parent: Option<AssetId>) -> Asset {
            unimplemented!()
        }

        async fn update_asset(&self, _id: AssetId, _changes: PartialAsset) -> Result<(), String> {
            unimplemented!()
        }

        async fn delete_asset(&self, id: AssetId) -> Result<(), String> {
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
        assert!(execute(&port, AssetId::from(1)).await.is_ok());
    }

    #[tokio::test]
    async fn delete_missing() {
        let port = MockAssetCommand;
        assert!(execute(&port, AssetId::from(999)).await.is_err());
    }
}
