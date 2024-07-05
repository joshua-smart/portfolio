use anyhow::Result;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

mod index;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    let app = Router::new()
        .route("/", get(|| async { index::IndexTemplate }))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(TraceLayer::new_for_http());

    let address = "0.0.0.0";
    let port = 3000;

    let listener = TcpListener::bind((address, port)).await?;
    info!("Starting server on `{address}:{port}`");
    axum::serve(listener, app).await?;

    Ok(())
}
