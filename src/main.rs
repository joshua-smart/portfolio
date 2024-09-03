use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use clap::Parser;
use data::Data;
use sqlx::{migrate, SqlitePool};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::args::Args;

mod args;
mod data;
mod filters;
mod navigation;
mod not_found;
mod pages;
mod partials;

#[derive(Clone)]
struct AppState {
    data: Arc<Data>,
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
        data_path,
    } = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .compact()
        .init();

    let data_file_contents = tokio::fs::read_to_string(data_path).await?;
    let data = ron::from_str::<data::Data>(&data_file_contents)?;

    let pool = SqlitePool::connect(&database_url).await?;
    migrate!().run(&pool).await?;

    let state = AppState {
        db: pool,
        data: Arc::new(data),
    };

    let app = Router::<AppState>::new()
        .nest("/partials", partials::router())
        .nest("/", pages::router())
        .fallback(not_found::get)
        .with_state(state)
        .nest_service("/assets", ServeDir::new(asset_dir)) // Serve static assets
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind((address.as_str(), port)).await?;
    info!(address, port, "Starting server");
    axum::serve(listener, app).await?;

    Ok(())
}
