use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use sqlx::{migrate, SqlitePool};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::args::Args;

mod args;
mod index;
mod partials;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .compact()
        .init();

    let pool = SqlitePool::connect(&args.database_url).await?;
    migrate!().run(&pool).await?;

    let state = AppState { db: pool };

    let app = Router::<AppState>::new()
        .route("/", get(index::get))
        .nest("/partials", partials::router())
        .with_state(state)
        .nest_service("/assets", ServeDir::new(args.asset_dir))
        .layer(TraceLayer::new_for_http());

    let address = "0.0.0.0";
    let port = 3000;

    let listener = TcpListener::bind((address, port)).await?;
    info!("Starting server on `{address}:{port}`");
    axum::serve(listener, app).await?;

    Ok(())
}
