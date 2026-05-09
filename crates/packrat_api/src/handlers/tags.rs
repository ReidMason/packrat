use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use packrat_application::{ensure_tag, list_tags, set_asset_tags};
use packrat_domain::PermissionSlug;
use packrat_domain::asset::AssetId;
use packrat_domain::tag::{TagId, TagName};
use packrat_domain::tenant::TenantId;

use crate::authorization::ensure_tenant_permission;
use crate::dto::{CreateTagDto, ErrorBody, ListTagsQuery, SetAssetTagsDto, SuccessBody, TagDto};
use crate::middleware::AuthSession;
use crate::state::AppState;

pub async fn list_tags_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path(tenant_id): Path<i64>,
    Query(query): Query<ListTagsQuery>,
) -> Result<Json<SuccessBody<Vec<TagDto>>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsRead).await?;
    let prefix = query.q.as_deref().and_then(|s| {
        let t = s.trim();
        if t.is_empty() { None } else { Some(t) }
    });
    let tags = list_tags(state.tags.as_ref(), TenantId::from(tenant_id), prefix).await;
    Ok(Json(SuccessBody::new(
        tags.into_iter().map(TagDto::from_tag).collect(),
    )))
}

pub async fn ensure_tag_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path(tenant_id): Path<i64>,
    Json(body): Json<CreateTagDto>,
) -> Result<Json<SuccessBody<TagDto>>, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsWrite).await?;
    let name = TagName::parse(&body.name)
        .map_err(|m| (StatusCode::BAD_REQUEST, Json(ErrorBody::message(m))))?;
    let tag = ensure_tag(state.tags.as_ref(), TenantId::from(tenant_id), name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::message(e)),
            )
        })?;
    Ok(Json(SuccessBody::new(TagDto::from_tag(tag))))
}

pub async fn set_asset_tags_handler(
    State(state): State<AppState>,
    Extension(session): Extension<AuthSession>,
    Path((tenant_id, id)): Path<(i64, i64)>,
    Json(body): Json<SetAssetTagsDto>,
) -> Result<StatusCode, (StatusCode, Json<ErrorBody>)> {
    ensure_tenant_permission(&state, &session, tenant_id, PermissionSlug::AssetsWrite).await?;
    let tag_ids: Vec<TagId> = body.tag_ids.into_iter().map(TagId::from).collect();
    set_asset_tags(
        state.tags.as_ref(),
        TenantId::from(tenant_id),
        AssetId::from(id),
        tag_ids,
    )
    .await
    .map_err(|e| {
        let status = if e.contains("not found") {
            StatusCode::NOT_FOUND
        } else if e.contains("another tenant") || e.contains("not found for this tenant") {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, Json(ErrorBody::message(e)))
    })?;
    Ok(StatusCode::NO_CONTENT)
}
