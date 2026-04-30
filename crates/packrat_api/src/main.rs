//! Axum HTTP server: JSON REST surface over `packrat_application` ports.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use packrat_application::{create_item, get_item, ItemCommandPort};
use packrat_domain::entity::{Entity, EntityId, EntityName};
use packrat_infrastructure::{
    PostgresItemCommand, PostgresItemQuery, connect_pool, run_migrations,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    command: Arc<PostgresItemCommand>,
    query: Arc<PostgresItemQuery>,
}

#[derive(serde::Deserialize)]
struct CreateItemBody {
    name: String,
    #[serde(default)]
    parent_id: Option<i64>,
}

#[derive(Serialize)]
struct ItemResponse {
    id: i64,
    name: String,
    parent_id: Option<i64>,
    created: String,
    deleted: Option<String>,
}

impl ItemResponse {
    fn from_entity(e: Entity) -> Self {
        Self {
            id: i64::from(e.id),
            name: e.name.as_str().to_string(),
            parent_id: e.parent.map(i64::from),
            created: e.created.to_string(),
            deleted: e.deleted.map(|d| d.to_string()),
        }
    }
}

async fn create_item_handler(
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

async fn get_item_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ItemResponse>, StatusCode> {
    get_item(state.query.as_ref(), EntityId::from(id))
        .await
        .map(|e| Json(ItemResponse::from_entity(e)))
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_item_handler(
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let listen = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());

    let pool = connect_pool(&database_url).await?;
    run_migrations(&pool).await?;

    let state = AppState {
        command: Arc::new(PostgresItemCommand::new(pool.clone())),
        query: Arc::new(PostgresItemQuery::new(pool)),
    };

    let api = Router::new()
        .route("/items", post(create_item_handler))
        .route("/items/{id}", get(get_item_handler))
        .route("/items/{id}", delete(delete_item_handler))
        .with_state(state);

    let app = Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&listen).await?;
    tracing::info!("listening on http://{}", listen);
    axum::serve(listener, app).await?;

    Ok(())
}
