use std::path::Path;

use axum::{response::Redirect, routing::get, Router};
use tower_http::services::ServeFile;

use crate::AppState;

mod cv;
mod index;

pub fn router(asset_dir: &Path) -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .route_service(
            "/resume",
            ServeFile::new(asset_dir.join("CV-Joshua-Smart.pdf")),
        )
        .route("/cv", get(|| async { Redirect::permanent("/resume") }))
        .route("/health", get(health))
}

async fn health() -> String {
    "healthy".to_string()
}
