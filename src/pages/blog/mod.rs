use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode, Router};
use sqlx::query_as;
use tracing::error;

use crate::filters;
use crate::AppState;

mod edit;
mod post;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:post", axum::routing::get(post::get))
        .route(
            "/:post/edit",
            axum::routing::get(edit::get).post(edit::post),
        )
}

async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let posts = query_as!(Post, r#"SELECT id as "id: u32", title, written FROM posts"#)
        .fetch_all(&db)
        .await
        .map_err(|e| {
            error!(info = ?e, "Error retrieving posts from database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let index = BlogTemplate { posts };

    Ok(index)
}

#[derive(Debug)]
struct Post {
    id: u32,
    title: String,
    written: String,
}

#[derive(Template)]
#[template(path = "pages/blog/index.html")]
struct BlogTemplate {
    posts: Vec<Post>,
}
