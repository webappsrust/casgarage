pub mod acme;
pub mod admin;
pub mod auth;
pub mod cli;
pub mod config;
pub mod db;
pub mod garage;
pub mod logging;
pub mod s3;
pub mod scheduler;
pub mod storage;
pub mod web;

use anyhow::Result;
use cli::Cli;

/// Main entry point for the application
pub async fn run(cli: Cli) -> Result<()> {
    cli.execute().await
}

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
