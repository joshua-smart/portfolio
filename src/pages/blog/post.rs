use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query_as;
use tracing::error;

use crate::AppState;

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let post = query_as!(
        PostTemplate,
        r#"
        SELECT id as "id: u32", title, content, written
        FROM posts 
        WHERE id=?"#,
        id,
    )
    .fetch_one(&db)
    .await
    .map_err(|e| {
        error!(info = ?e, "Error retrieving blog page from database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(post)
}

#[derive(Template)]
#[template(path = "pages/blog/post/index.html")]
struct PostTemplate {
    id: u32,
    title: String,
    content: String,
    written: String,
}
