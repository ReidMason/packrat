use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use packrat_application::{ItemCommandPort, create_item, get_item};
use packrat_domain::entity::{EntityId, EntityName};

use crate::dto::{CreateItemDto, ItemDto};
use crate::state::AppState;

pub async fn create_item_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateItemDto>,
) -> Result<(StatusCode, Json<ItemDto>), (StatusCode, String)> {
    if body.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "name must not be empty".into()));
    }
    let entity = create_item(
        state.command.as_ref(),
        EntityName::from(body.name),
        body.parent_id.map(EntityId::from),
    )
    .await;
    Ok((StatusCode::CREATED, Json(ItemDto::from_entity(entity))))
}

pub async fn get_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ItemDto>, StatusCode> {
    get_item(state.query.as_ref(), EntityId::from(id))
        .await
        .map(|e| Json(ItemDto::from_entity(e)))
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
