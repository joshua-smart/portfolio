use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode};
use futures::TryStreamExt;
use sqlx::{query, query_as};
use tracing::error;

use crate::AppState;

pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let db = state.db;

    let get_tools = {
        let db = &db;
        move |id: u32| async move {
            query_as!(
                Tool,
                r#"SELECT tools.name, tools.link 
                FROM projects 
                INNER JOIN project_tools ON projects.id = project_tools.project_id
                INNER JOIN tools ON project_tools.tool_id = tools.id
                WHERE projects.id = ?"#,
                id
            )
            .fetch_all(db)
            .await
        }
    };

    let projects: Vec<_> = query!(
        r#"
        SELECT projects.id as "id: u32", name, type, link 
        FROM projects
        INNER JOIN sources ON projects.source_id = sources.id
        "#
    )
    .fetch(&db)
    .and_then(|r| async move {
        let tools = get_tools(r.id).await?;

        Ok(Project {
            id: r.id,
            name: r.name,
            tools,
            source: Source {
                r#type: r.r#type,
                link: r.link,
            },
        })
    })
    .try_collect()
    .await
    .map_err(|e| {
        error!(info = ?e, "Error retrieving projects from database");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(ProjectCardsTemplate { projects })
}

#[derive(Debug)]
struct Project {
    id: u32,
    name: String,
    tools: Vec<Tool>,
    source: Source,
}

#[derive(Debug)]
struct Tool {
    name: String,
    link: String,
}

#[derive(Debug)]
struct Source {
    r#type: String,
    link: String,
}

#[derive(Template)]
#[template(path = "partials/project-cards.html")]
struct ProjectCardsTemplate {
    projects: Vec<Project>,
}
