use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use sqlx::{migrate, SqlitePool};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::args::Args;

mod args;
mod blog;
mod index;
mod partials;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        log_level,
        database_url,
        asset_dir,
        port,
        address,
    } = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .compact()
        .init();

    let pool = SqlitePool::connect(&database_url).await?;
    migrate!().run(&pool).await?;

    let state = AppState { db: pool };

    let app = Router::<AppState>::new()
        .route("/", get(index::get))
        .nest("/partials", partials::router())
        .nest("/blog", blog::router())
        .with_state(state)
        .nest_service("/assets", ServeDir::new(asset_dir)) // Serve static assets
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind((address.as_str(), port)).await?;
    info!(address, port, "Starting server");
    axum::serve(listener, app).await?;

    Ok(())
}
