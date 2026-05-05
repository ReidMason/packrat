use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use packrat_application::{
    AssetCommandPort, AssetSearchQuery, create_asset, get_asset, list_assets, list_child_assets,
    search_assets,
};
use packrat_domain::entity::{EntityId, EntityName};

use crate::dto::{AssetDto, CreateAssetDto, ErrorBody, SearchAssetsDto, SuccessBody};
use crate::state::AppState;

pub async fn list_child_assets_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Json<SuccessBody<Vec<AssetDto>>> {
    let entities = list_child_assets(state.query.as_ref(), EntityId::from(id)).await;
    Json(SuccessBody::new(
        entities.into_iter().map(AssetDto::from_entity).collect(),
    ))
}

pub async fn search_assets_handler(
    State(state): State<AppState>,
    Json(body): Json<SearchAssetsDto>,
) -> Result<Json<SuccessBody<Vec<AssetDto>>>, (StatusCode, Json<ErrorBody>)> {
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
    let query = AssetSearchQuery { name, fuzzyname };
    let entities = search_assets(state.query.as_ref(), &query).await;
    Ok(Json(SuccessBody::new(
        entities.into_iter().map(AssetDto::from_entity).collect(),
    )))
}

pub async fn list_assets_handler(
    State(state): State<AppState>,
) -> Json<SuccessBody<Vec<AssetDto>>> {
    let entities = list_assets(state.query.as_ref()).await;
    let dtos: Vec<AssetDto> = entities.into_iter().map(AssetDto::from_entity).collect();
    Json(SuccessBody::new(dtos))
}

pub async fn create_asset_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateAssetDto>,
) -> Result<(StatusCode, Json<SuccessBody<AssetDto>>), (StatusCode, Json<ErrorBody>)> {
    if body.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("name must not be empty")),
        ));
    }
    let entity = create_asset(
        state.command.as_ref(),
        EntityName::from(body.name),
        body.parent_id.map(EntityId::from),
    )
    .await;
    Ok((
        StatusCode::CREATED,
        Json(SuccessBody::new(AssetDto::from_entity(entity))),
    ))
}

pub async fn get_asset_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<SuccessBody<AssetDto>>, (StatusCode, Json<ErrorBody>)> {
    match get_asset(state.query.as_ref(), EntityId::from(id)).await {
        Some(e) => Ok(Json(SuccessBody::new(AssetDto::from_entity(e)))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorBody::message("asset not found")),
        )),
    }
}

pub async fn delete_asset_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorBody>)> {
    state
        .command
        .delete_asset(EntityId::from(id))
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
