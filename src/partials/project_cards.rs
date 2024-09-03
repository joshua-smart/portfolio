use std::collections::BTreeMap;

use crate::{data::Project, filters};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode};

use crate::AppState;

pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let projects = state.data.projects.clone();

    Ok(ProjectCardsTemplate { projects })
}

use crate::data::Source;

#[derive(Template)]
#[template(path = "partials/project-cards.html")]
struct ProjectCardsTemplate {
    projects: BTreeMap<usize, Project>,
}
