use packrat_application::AssetWithTags;
use packrat_domain::asset::Asset;
use serde::Serialize;

use super::tags::TagDto;

#[derive(serde::Deserialize)]
pub struct CreateAssetDto {
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct SearchAssetsDto {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub fuzzyname: Option<String>,
}

#[derive(Serialize)]
pub struct AssetDto {
    pub id: i64,
    pub tenant_id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagDto>,
}

impl AssetDto {
    pub fn from_asset_with_tags(e: AssetWithTags) -> Self {
        Self {
            id: i64::from(e.asset.id),
            tenant_id: i64::from(e.asset.tenant_id),
            name: e.asset.name.as_str().to_string(),
            parent_id: e.asset.parent.map(i64::from),
            created: e.asset.created.to_string(),
            deleted: e.asset.deleted.map(|d| d.to_string()),
            tags: e.tags.into_iter().map(TagDto::from_tag).collect(),
        }
    }

    pub fn from_entity(e: Asset) -> Self {
        Self::from_asset_with_tags(AssetWithTags::new(e, vec![]))
    }
}
