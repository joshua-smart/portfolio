use axum::{routing::get, Router};

use crate::AppState;

mod cv;
mod index;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index::get))
        .route("/cv", get(cv::get))
}
