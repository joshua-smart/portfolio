use askama_axum::{IntoResponse, Template};

pub async fn get() -> impl IntoResponse {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate;
