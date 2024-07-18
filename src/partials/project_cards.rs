use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode};
use futures::TryStreamExt;
use sqlx::{query, query_scalar};
use tracing::error;

use crate::AppState;

pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let get_tools = {
        let db = db.clone();
        |id: u32| async move {
            query_scalar!(r#"
            SELECT tools.name as "tool: Tool" 
            FROM projects, tools, project_tools 
            WHERE projects.id = project_tools.project_id AND tools.id = project_tools.tool_id AND projects.id = ?"#,
            id)
            .fetch_all(&db).await
        }
    };

    let projects: Vec<_> = query!(r#"SELECT id as "id: u32", name FROM projects"#)
        .fetch(&db)
        .and_then(|r| {
            let get_tools = get_tools.clone();
            async move {
                let tools = get_tools(r.id).await?;

                Ok(Project {
                    id: r.id,
                    name: r.name,
                    tools,
                })
            }
        })
        .try_collect()
        .await
        .map_err(|e| {
            error!(info = ?e, "Error retrieving projects from database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(ProjectCardsTemplate { projects })
}

struct Project {
    id: u32,
    name: String,
    tools: Vec<Tool>,
}

#[derive(sqlx::Type, Debug)]
enum Tool {
    Rust,
}

#[derive(Template)]
#[template(path = "partials/project-cards.html")]
struct ProjectCardsTemplate {
    projects: Vec<Project>,
}
