use std::path::PathBuf;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn apply(router: Router, root: PathBuf) -> Router {
    let index_path = root.join("index.html");
    if !(root.is_dir() && index_path.is_file()) {
        tracing::warn!(
            path = %root.display(),
            "static UI path is not a directory with index.html; serving API routes only"
        );
        return router;
    }

    tracing::info!(path = %root.display(), "also serving static UI");
    router.fallback_service(ServeDir::new(&root).fallback(ServeFile::new(index_path)))
}
