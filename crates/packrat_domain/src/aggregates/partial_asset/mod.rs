use crate::asset::{AssetId, AssetName};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct PartialAsset {
    pub name: Option<AssetName>,
    pub parent: Option<Option<AssetId>>,
}

impl PartialAsset {
    pub fn new(name: Option<AssetName>, parent: Option<Option<AssetId>>) -> Self {
        Self { name, parent }
    }
}
