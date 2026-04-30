mod app;
mod dto;
mod handlers;
mod state;

use std::sync::Arc;

use packrat_infrastructure::{
    PostgresItemCommand, PostgresItemQuery, connect_pool, run_migrations,
};

use crate::state::AppState;

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

    let app = app::build_app(state);

    let listener = tokio::net::TcpListener::bind(&listen).await?;
    tracing::info!("listening on http://{}", listen);

    let shutdown = async {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("shutdown signal received, finishing in-flight requests");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;

    Ok(())
}
