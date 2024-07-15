use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use sqlx::query_as;

use crate::AppState;

pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db;

    let Ok(projects) = query_as!(Project, r"SELECT name FROM projects")
        .fetch_all(&db)
        .await
    else {
        return Err("Oops".to_string());
    };

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
