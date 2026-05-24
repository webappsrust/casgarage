//! Server start command implementation

use anyhow::Result;
use crate::cli::Cli;
use crate::config::Config;

/// Start the server
pub async fn run(cli: Cli) -> Result<()> {
    // Create configuration
    let config = Config::from_cli(cli)?;

    // Initialize logging system
    crate::logging::init_logging(&config)?;

    // Print startup banner
    print_banner(&config);

    // Initialize database
    let db = crate::db::Database::new(&config).await?;

    // Start web server
    crate::web::server::start(config, db).await?;

    Ok(())
}

/// Print startup banner
fn print_banner(config: &Config) {
    println!("\n{}", "=".repeat(60));
    println!("  🚗 CasGarage v{}", env!("CARGO_PKG_VERSION"));
    println!("  S3-Compatible Object Storage");
    println!("{}", "=".repeat(60));
    println!("\n📊 Server Configuration:");
    println!("  • Address: {}", config.display_url());
    println!("  • Data: {}", config.data_dir.display());
    println!("  • Config: {}", config.config_dir.display());
    println!("  • Logs: {}", config.log_dir.display());
    println!("\n{}", "=".repeat(60));
    println!();
}
