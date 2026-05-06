use packrat_domain::asset::{Asset, AssetId, AssetName};

use crate::ports::AssetCommandPort;

pub async fn execute(
    port: &impl AssetCommandPort,
    name: AssetName,
    parent: Option<AssetId>,
) -> Asset {
    port.create_asset(name, parent).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::{aggregates::partial_asset::PartialAsset, asset::AssetTimestamp};

    struct MockAssetCommand;

    #[async_trait]
    impl AssetCommandPort for MockAssetCommand {
        async fn create_asset(&self, name: AssetName, parent: Option<AssetId>) -> Asset {
            let created = AssetTimestamp::now();
            let deleted = None;
            Asset::new(AssetId::from(99), name, parent, created, deleted)
        }
        async fn update_asset(&self, _id: AssetId, _changes: PartialAsset) -> Result<(), String> {
            unimplemented!()
        }
        async fn delete_asset(&self, _id: AssetId) -> Result<(), String> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn execute_creates_asset_via_port() {
        let port = MockAssetCommand;
        let parent = Some(AssetId::from(1));
        let asset = execute(&port, AssetName::from("alpha"), parent).await;
        assert_eq!(asset.id, AssetId::from(99));
        assert_eq!(asset.name, AssetName::from("alpha"));
        assert_eq!(asset.parent, parent);
    }

    #[tokio::test]
    async fn execute_creates_root_asset() {
        let port = MockAssetCommand;
        let asset = execute(&port, AssetName::from("root"), None).await;
        assert_eq!(asset.parent, None);
    }
}
