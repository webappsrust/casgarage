//! Comprehensive logging system
//!
//! Provides structured logging with multiple formats:
//! - Apache Common Log Format (access.log) - configurable via admin UI
//! - JSON structured logs (application.log)
//! - Error logs (error.log)
//! - Audit logs (audit.log)

pub mod access;
pub mod audit;
pub mod formatters;

use anyhow::Result;
use std::path::Path;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::Config;

/// Initialize logging system
pub fn init_logging(config: &Config) -> Result<()> {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all(&config.log_dir)?;

    // Set up log files
    let access_log = config.log_dir.join("access.log");
    let error_log = config.log_dir.join("error.log");
    let app_log = config.log_dir.join("application.log");

    // Create file appenders
    let access_appender = tracing_appender::rolling::daily(&config.log_dir, "access.log");
    let error_appender = tracing_appender::rolling::daily(&config.log_dir, "error.log");
    let app_appender = tracing_appender::rolling::daily(&config.log_dir, "application.log");

    // Configure filter from environment or default
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,casgarage=debug,tower_http=debug"));

    // Build subscriber with multiple layers
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(app_appender)
                .with_ansi(false)
                .json(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(error_appender)
                .with_ansi(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::from_level(Level::WARN)),
        )
        .init();

    tracing::info!("📝 Logging system initialized");
    tracing::info!("  ├─ Access log: {}", access_log.display());
    tracing::info!("  ├─ Error log: {}", error_log.display());
    tracing::info!("  └─ Application log: {}", app_log.display());

    Ok(())
}

/// Log level enum
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}
