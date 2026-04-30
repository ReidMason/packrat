use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use packrat_application::{create_item, get_item, ItemCommandPort};
use packrat_domain::entity::{Entity, EntityId, EntityName};
use serde::Serialize;

use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct CreateItemBody {
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
}

impl ItemResponse {
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

pub async fn create_item_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateItemBody>,
) -> Result<(StatusCode, Json<ItemResponse>), (StatusCode, String)> {
    if body.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "name must not be empty".into()));
    }
    let entity = create_item(
        state.command.as_ref(),
        EntityName::from(body.name),
        body.parent_id.map(EntityId::from),
    )
    .await;
    Ok((StatusCode::CREATED, Json(ItemResponse::from_entity(entity))))
}

pub async fn get_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ItemResponse>, StatusCode> {
    get_item(state.query.as_ref(), EntityId::from(id))
        .await
        .map(|e| Json(ItemResponse::from_entity(e)))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn delete_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .command
        .delete_entity(EntityId::from(id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.contains("children") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, e)
        })
}
