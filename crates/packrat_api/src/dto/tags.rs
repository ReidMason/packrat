use packrat_domain::tag::Tag;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TagDto {
    pub id: i64,
    pub name: String,
}

impl TagDto {
    pub fn from_tag(tag: Tag) -> Self {
        Self {
            id: i64::from(tag.id),
            name: tag.name.as_str().to_string(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateTagDto {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct SetAssetTagsDto {
    #[serde(default)]
    pub tag_ids: Vec<i64>,
}

#[derive(serde::Deserialize)]
pub struct SearchTagsDto {
    #[serde(default)]
    pub prefix: Option<String>,
}
