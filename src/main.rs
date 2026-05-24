use anyhow::Result;
use casgarage::cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Execute command
    // Logging will be initialized by the server command
    casgarage::run(cli).await?;

    Ok(())
}
