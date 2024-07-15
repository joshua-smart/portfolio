use std::path::PathBuf;

use tracing::Level;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(env, long, short, default_value = "INFO")]
    pub log_level: Level,

    #[arg(env, long, default_value = "assets/")]
    pub asset_dir: PathBuf,

    #[arg(env, long, default_value = "sqlite::memory:")]
    pub database_url: String,
}
