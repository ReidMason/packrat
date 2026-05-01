use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use packrat_application::{create_item, get_item, list_items, search_items, ItemCommandPort, ItemSearchQuery};
use packrat_domain::entity::{EntityId, EntityName};

use crate::dto::{CreateItemDto, ErrorBody, ItemDto, SearchItemsDto, SuccessBody};
use crate::state::AppState;

pub async fn search_items_handler(
    State(state): State<AppState>,
    Json(body): Json<SearchItemsDto>,
) -> Result<Json<SuccessBody<Vec<ItemDto>>>, (StatusCode, Json<ErrorBody>)> {
    let name = body
        .name
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let fuzzyname = body
        .fuzzyname
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    if name.is_none() && fuzzyname.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message(
                "at least one of name or fuzzyname must be a non-empty string",
            )),
        ));
    }
    let query = ItemSearchQuery { name, fuzzyname };
    let entities = search_items(state.query.as_ref(), &query).await;
    Ok(Json(SuccessBody::new(
        entities.into_iter().map(ItemDto::from_entity).collect(),
    )))
}

pub async fn list_items_handler(
    State(state): State<AppState>,
) -> Json<SuccessBody<Vec<ItemDto>>> {
    let entities = list_items(state.query.as_ref()).await;
    let dtos: Vec<ItemDto> = entities
        .into_iter()
        .map(ItemDto::from_entity)
        .collect();
    Json(SuccessBody::new(dtos))
}

pub async fn create_item_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateItemDto>,
) -> Result<
    (StatusCode, Json<SuccessBody<ItemDto>>),
    (StatusCode, Json<ErrorBody>),
> {
    if body.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("name must not be empty")),
        ));
    }
    let entity = create_item(
        state.command.as_ref(),
        EntityName::from(body.name),
        body.parent_id.map(EntityId::from),
    )
    .await;
    Ok((
        StatusCode::CREATED,
        Json(SuccessBody::new(ItemDto::from_entity(entity))),
    ))
}

pub async fn get_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<SuccessBody<ItemDto>>, (StatusCode, Json<ErrorBody>)> {
    match get_item(state.query.as_ref(), EntityId::from(id)).await {
        Some(e) => Ok(Json(SuccessBody::new(ItemDto::from_entity(e)))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorBody::message("item not found")),
        )),
    }
}

pub async fn delete_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorBody>)> {
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
            (status, Json(ErrorBody::message(e)))
        })
}
