use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post, put};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers::assets::{
    create_asset_handler, delete_asset_handler, get_asset_handler, list_assets_handler,
    list_child_assets_handler, search_assets_handler,
};
use crate::handlers::auth::login_handler;
use crate::handlers::health::health_handler;
use crate::handlers::ready::ready_handler;
use crate::handlers::tags::{ensure_tag_handler, search_tags_handler, set_asset_tags_handler};
use crate::handlers::tenants::{create_tenant_handler, list_my_tenants_handler};
use crate::handlers::users::create_user_handler;
use crate::middleware::require_session;
use crate::state::AppState;
use crate::static_ui;

fn api_router(state: AppState) -> Router {
    let public = Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/users", post(create_user_handler))
        .route("/login", post(login_handler))
        .with_state(state.clone());

    let protected = Router::new()
        .route("/tenants", post(create_tenant_handler))
        .route("/me/tenants", get(list_my_tenants_handler))
        .route(
            "/tenants/{tenant_id}/tags/search",
            post(search_tags_handler),
        )
        .route("/tenants/{tenant_id}/tags", post(ensure_tag_handler))
        .route(
            "/tenants/{tenant_id}/assets/search",
            post(search_assets_handler),
        )
        .route(
            "/tenants/{tenant_id}/assets/{id}/tags",
            put(set_asset_tags_handler),
        )
        .route(
            "/tenants/{tenant_id}/assets",
            get(list_assets_handler).post(create_asset_handler),
        )
        .route(
            "/tenants/{tenant_id}/assets/{id}/children",
            get(list_child_assets_handler),
        )
        .route(
            "/tenants/{tenant_id}/assets/{id}",
            get(get_asset_handler).delete(delete_asset_handler),
        )
        .layer(from_fn_with_state(state.clone(), require_session))
        .with_state(state.clone());

    Router::new().merge(public).merge(protected)
}

pub fn build_app(state: AppState, static_ui: Option<PathBuf>) -> Router {
    let api = Router::new().nest("/api", api_router(state));

    let router = match static_ui {
        Some(root) => static_ui::apply(api, root),
        None => api,
    };

    router
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
