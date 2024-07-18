use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode};
use sqlx::query_as;
use tracing::error;

use crate::AppState;

pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let projects = query_as!(Project, r"SELECT * FROM projects")
        .fetch_all(&db)
        .await
        .map_err(|e| {
            error!(info = ?e, "Error retrieving projects from database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(ProjectCardsTemplate { projects })
}

struct Project {
    name: String,
}

#[derive(Template)]
#[template(path = "partials/project-cards.html")]
struct ProjectCardsTemplate {
    projects: Vec<Project>,
}
