pub mod commands;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "casgarage")]
#[command(about = "🚗 CasGarage - Self-hosted S3-compatible object storage", long_about = None)]
#[command(version = VERSION)]
pub struct Cli {
    /// Server port (single port for HTTP, or "8080,8443" for HTTP,HTTPS)
    #[arg(long, env = "PORT")]
    pub port: Option<String>,

    /// Data directory
    #[arg(long, env = "DATA_DIR")]
    pub datadir: Option<PathBuf>,

    /// Config directory
    #[arg(long, env = "CONFIG_DIR")]
    pub configdir: Option<PathBuf>,

    /// Log directory
    #[arg(long, env = "LOG_DIR")]
    pub logdir: Option<PathBuf>,

    /// Listen address (e.g., "0.0.0.0", "192.168.1.100", "example.com")
    #[arg(long, env = "SERVER_ADDRESS")]
    pub address: Option<String>,

    /// Show status and health (exit with appropriate code)
    #[arg(long)]
    pub status: bool,
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        if self.status {
            return commands::health::run().await;
        }

        // Start server with provided options
        commands::server::run(self).await
    }
}
