use askama::Template;
use askama_axum::IntoResponse;

pub async fn get() -> impl IntoResponse {
    CVTemplate
}

#[derive(Template)]
#[template(path = "pages/cv.html")]
struct CVTemplate;
