use axum::{http::StatusCode, routing::get, Router};

use crate::AppState;

mod project_cards;
mod timeline;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/project-cards", get(project_cards::get))
        .route("/timeline", get(timeline::get))
        .fallback(|| async { StatusCode::NOT_FOUND })
}
