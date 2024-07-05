use axum::{routing::get, Router};
use anyhow::Result;

mod index;

#[tokio::main]
async fn main() -> Result<()> {

    let app = Router::new()
        .route("/", get(|| async { index::IndexTemplate }))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
