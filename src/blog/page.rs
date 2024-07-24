use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query;
use tracing::error;

use crate::AppState;

pub async fn get(
    State(state): State<AppState>,
    Path(page): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let page_data = query!(
        r#"
        SELECT id as "id: u32", content, written
        FROM blogs 
        WHERE title=?"#,
        page,
    )
    .fetch_one(&db)
    .await
    .map_err(|e| {
        error!(info = ?e, "Error retrieving blog page from database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let page = PageTemplate {
        id: page_data.id,
        title: page,
        content: page_data.content,
        written: page_data.written,
    };

    Ok(page)
}

#[derive(Template)]
#[template(path = "pages/blog/page.html")]
struct PageTemplate {
    id: u32,
    title: String,
    content: String,
    written: String,
}
