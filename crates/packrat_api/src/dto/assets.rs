use packrat_domain::entity::Entity;
use serde::Serialize;

/// **DTO** — JSON body for `POST /api/assets`.
#[derive(serde::Deserialize)]
pub struct CreateAssetDto {
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
}

/// **DTO** — JSON body for `POST /api/assets/search`.
#[derive(serde::Deserialize)]
pub struct SearchAssetsDto {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub fuzzyname: Option<String>,
}

/// **DTO** — `data` payload inside [`SuccessBody`](super::SuccessBody) for asset reads/creates.
#[derive(Serialize)]
pub struct AssetDto {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
}

impl AssetDto {
    pub fn from_entity(e: Entity) -> Self {
        Self {
            id: i64::from(e.id),
            name: e.name.as_str().to_string(),
            parent_id: e.parent.map(i64::from),
            created: e.created.to_string(),
            deleted: e.deleted.map(|d| d.to_string()),
        }
    }
}
