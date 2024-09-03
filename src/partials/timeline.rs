use std::collections::BTreeMap;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode};

use crate::data::Event;
use crate::filters;
use crate::AppState;

pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let events = state.data.events.clone();

    Ok(TimelineTemplate { events })
}

#[derive(Template)]
#[template(path = "partials/timeline.html")]
struct TimelineTemplate {
    events: BTreeMap<usize, Event>,
}
