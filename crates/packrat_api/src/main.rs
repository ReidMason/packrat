mod app;
mod authorization;
mod dto;
mod handlers;
mod middleware;
mod state;
mod static_ui;

use std::path::PathBuf;
use std::sync::Arc;

use packrat_infrastructure::{
    PostgresAssetCommand, PostgresAssetQuery, PostgresAuthorizationQuery, PostgresReadiness,
    PostgresTags, PostgresTenantCommand, PostgresTenantQuery, PostgresUserCommand,
    PostgresUserQuery, PostgresUserSessionCommand, PostgresUserSessionQuery, connect_pool,
    run_migrations,
};

use crate::state::AppState;

fn static_ui_dir() -> Option<PathBuf> {
    std::env::var("PACKRAT_STATIC_UI").ok().and_then(|s| {
        let p = PathBuf::from(s);
        if p.is_dir() && p.join("index.html").is_file() {
            Some(p)
        } else {
            tracing::warn!(
                path = %p.display(),
                "PACKRAT_STATIC_UI is set but is not a directory with index.html; serving API routes only"
            );
            None
        }
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

    let static_ui = static_ui_dir();

    let state = AppState {
        readiness: PostgresReadiness::new(pool.clone()),
        command: Arc::new(PostgresAssetCommand::new(pool.clone())),
        query: Arc::new(PostgresAssetQuery::new(pool.clone())),
        user_command: Arc::new(PostgresUserCommand::new(pool.clone())),
        user_query: Arc::new(PostgresUserQuery::new(pool.clone())),
        user_session_command: Arc::new(PostgresUserSessionCommand::new(pool.clone())),
        user_session_query: Arc::new(PostgresUserSessionQuery::new(pool.clone())),
        tenant_command: Arc::new(PostgresTenantCommand::new(pool.clone())),
        tenant_query: Arc::new(PostgresTenantQuery::new(pool.clone())),
        authorization: Arc::new(PostgresAuthorizationQuery::new(pool.clone())),
        tags: Arc::new(PostgresTags::new(pool)),
    };

    let app = app::build_app(state, static_ui);

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
