use std::path::PathBuf;

use tracing::Level;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(env, long, default_value = "INFO")]
    pub log_level: Level,

    #[arg(
        env,
        long,
        default_value = "assets/",
        help = "Directory to serve static assets from"
    )]
    pub asset_dir: PathBuf,

    #[arg(env, long, short = 'p', default_value = "3000")]
    pub port: u16,

    #[arg(env, long, short = 'a', default_value = "0.0.0.0")]
    pub address: String,

    #[arg(
        env,
        long,
        short = 'd',
        default_value = "data.ron",
        help = "Path to a file containing static data in RON format"
    )]
    pub data_path: PathBuf,
}
