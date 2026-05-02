use axum::Router;
use axum::routing::{get, post};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers::assets::{
    create_asset_handler, delete_asset_handler, get_asset_handler, list_assets_handler,
    list_child_assets_handler, search_assets_handler,
};
use crate::handlers::health::health_handler;
use crate::handlers::ready::ready_handler;
use crate::state::AppState;
use crate::static_ui;

fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/assets/search", post(search_assets_handler))
        .route(
            "/assets",
            get(list_assets_handler).post(create_asset_handler),
        )
        .route("/assets/{id}/children", get(list_child_assets_handler))
        .route(
            "/assets/{id}",
            get(get_asset_handler).delete(delete_asset_handler),
        )
        .with_state(state)
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
