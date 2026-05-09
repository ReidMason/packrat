use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use packrat_application::{
    AssetSearchQuery, create_asset, delete_asset, get_asset, list_assets, list_child_assets,
    search_assets,
};
use packrat_domain::asset::{AssetId, AssetName};
use packrat_domain::tenant::TenantId;
use packrat_domain::PermissionSlug;

use crate::authorization::ensure_tenant_permission;
use crate::dto::{AssetDto, CreateAssetDto, ErrorBody, SearchAssetsDto, SuccessBody};
use crate::middleware::AuthSession;
use crate::state::AppState;

pub async fn list_child_assets_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path((tenant_id, id)): Path<(i64, i64)>,
) -> Result<Json<SuccessBody<Vec<AssetDto>>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsRead).await?;
    let entities = list_child_assets(
        state.query.as_ref(),
        TenantId::from(tenant_id),
        AssetId::from(id),
    )
    .await;
    Ok(Json(SuccessBody::new(
        entities.into_iter().map(AssetDto::from_entity).collect(),
    )))
}

pub async fn search_assets_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path(tenant_id): Path<i64>,
    Json(body): Json<SearchAssetsDto>,
) -> Result<Json<SuccessBody<Vec<AssetDto>>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsRead).await?;
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
    let entities = search_assets(
        state.query.as_ref(),
        TenantId::from(tenant_id),
        &query,
    )
    .await;
    Ok(Json(SuccessBody::new(
        entities.into_iter().map(AssetDto::from_entity).collect(),
    )))
}

pub async fn list_assets_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path(tenant_id): Path<i64>,
) -> Result<Json<SuccessBody<Vec<AssetDto>>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsRead).await?;
    let entities = list_assets(state.query.as_ref(), TenantId::from(tenant_id)).await;
    let dtos: Vec<AssetDto> = entities.into_iter().map(AssetDto::from_entity).collect();
    Ok(Json(SuccessBody::new(dtos)))
}

pub async fn create_asset_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path(tenant_id): Path<i64>,
    Json(body): Json<CreateAssetDto>,
) -> Result<(StatusCode, Json<SuccessBody<AssetDto>>), (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsWrite).await?;
    if body.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("name must not be empty")),
        ));
    }
    let entity = create_asset(
        state.command.as_ref(),
        TenantId::from(tenant_id),
        AssetName::from(body.name),
        body.parent_id.map(AssetId::from),
    )
    .await
    .map_err(|e| {
        let status = if e.contains("parent") {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, Json(ErrorBody::message(e)))
    })?;
    Ok((
        StatusCode::CREATED,
        Json(SuccessBody::new(AssetDto::from_entity(entity))),
    ))
}

pub async fn get_asset_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path((tenant_id, id)): Path<(i64, i64)>,
) -> Result<Json<SuccessBody<AssetDto>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsRead).await?;
    match get_asset(
        state.query.as_ref(),
        TenantId::from(tenant_id),
        AssetId::from(id),
    )
    .await
    {
        Some(e) => Ok(Json(SuccessBody::new(AssetDto::from_entity(e)))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorBody::message("asset not found")),
        )),
    }
}

pub async fn delete_asset_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path((tenant_id, id)): Path<(i64, i64)>,
) -> Result<StatusCode, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsDelete).await?;
    delete_asset(
        state.command.as_ref(),
        TenantId::from(tenant_id),
        AssetId::from(id),
    )
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
