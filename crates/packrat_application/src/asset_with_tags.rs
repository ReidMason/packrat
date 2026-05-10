use packrat_domain::asset::Asset;
use packrat_domain::tag::Tag;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetWithTags {
    pub asset: Asset,
    pub tags: Vec<Tag>,
}

impl AssetWithTags {
    pub fn new(asset: Asset, tags: Vec<Tag>) -> Self {
        Self { asset, tags }
    }
}
