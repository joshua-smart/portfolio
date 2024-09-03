use askama::Template;
use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub async fn get() -> (StatusCode, impl IntoResponse) {
    (StatusCode::NOT_FOUND, NotFoundTemplate)
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
struct NotFoundTemplate;
