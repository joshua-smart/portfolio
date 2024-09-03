use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use clap::Parser;
use data::Data;
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        log_level,
        asset_dir,
        port,
        address,
        data_path,
    } = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .compact()
        .init();

    info!(path = ?data_path, "Reading data");
    let data_file_contents = tokio::fs::read_to_string(data_path)
        .await
        .context("Failed to read data")?;
    let data = ron::from_str::<data::Data>(&data_file_contents).context("Failed to parse data")?;

    let state = AppState {
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
