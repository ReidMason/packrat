use axum::routing::{delete, get};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers::health::health_handler;
use crate::handlers::items::{
    create_item_handler, delete_item_handler, get_item_handler, list_items_handler,
};
use crate::handlers::ready::ready_handler;
use crate::state::AppState;

fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/items", get(list_items_handler).post(create_item_handler))
        .route("/items/{id}", get(get_item_handler))
        .route("/items/{id}", delete(delete_item_handler))
        .with_state(state)
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", api_router(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
