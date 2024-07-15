use axum::{routing::get, Router};

use crate::AppState;

mod project_cards;

pub fn router() -> Router<AppState> {
    Router::new().route("/project-cards", get(project_cards::get))
}
