use askama::Template;
use askama_axum::IntoResponse;

pub async fn get() -> impl IntoResponse {
    NotFoundTemplate
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
struct NotFoundTemplate;
