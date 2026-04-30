use packrat_domain::entity::Entity;
use serde::Serialize;

#[derive(serde::Deserialize)]
pub struct CreateItemDto {
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ItemDto {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
}

impl ItemDto {
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
