use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Form,
};
use serde::Deserialize;
use sqlx::{query, query_as};
use tracing::error;

use crate::AppState;

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let post = query_as!(
        EditTemplate,
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
#[template(path = "pages/blog/post/edit.html")]
struct EditTemplate {
    id: u32,
    title: String,
    content: String,
    written: String,
}

#[derive(Deserialize)]
pub struct EditForm {
    content: String,
}

pub async fn post(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Form(form): Form<EditForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;
    let content = form.content;

    query!("UPDATE posts SET content=? WHERE id=?", content, id)
        .execute(&db)
        .await
        .map_err(|e| {
            error!(info = ?e, "Error updating post content");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let headers = [("HX-Redirect", format!("/blog/{id}"))];

    Ok(headers)
}
