use askama::Template;
use askama_axum::IntoResponse;
use axum::Router;

use crate::AppState;

mod page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:page", axum::routing::get(page::get))
}

async fn get() -> impl IntoResponse {
    BlogTemplate
}

#[derive(Template)]
#[template(path = "pages/blog/index.html")]
struct BlogTemplate;
