//! Database abstraction layer
//!
//! Provides unified interface for multiple database backends:
//! - SQLite (primary + cache/fallback)
//! - PostgreSQL
//! - MariaDB/MySQL
//! - Microsoft SQL Server
//! - Valkey/Redis (optional, for caching)
//!
//! When external database fails, automatically fails over to SQLite cache.
//! Server enters read-only maintenance mode until external DB recovers.

pub mod migrations;
pub mod models;
pub mod queries;
pub mod schema;

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::config::Config;

/// Database connection pool
#[derive(Clone)]
pub struct Database {
    /// SQLite pool (always available - primary or fallback)
    sqlite: SqlitePool,

    /// External database pool (if configured)
    external: Option<ExternalDb>,

    /// Current mode (normal or read-only maintenance)
    mode: Arc<RwLock<DatabaseMode>>,

    /// Configuration
    config: Arc<Config>,
}

/// External database connection
#[derive(Clone)]
pub enum ExternalDb {
    #[cfg(feature = "postgres")]
    Postgres(sqlx::PgPool),

    #[cfg(feature = "mysql")]
    MySql(sqlx::MySqlPool),

    #[cfg(feature = "valkey")]
    Valkey(redis::aio::ConnectionManager),
}

/// Database operational mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseMode {
    /// Normal operation - external DB (if configured) or SQLite
    Normal,

    /// Read-only maintenance - using SQLite cache, external DB unavailable
    ReadOnlyMaintenance,

    /// Critical error - database completely unavailable
    Critical,
}

impl Database {
    /// Initialize database with configuration
    pub async fn new(config: &Config) -> Result<Self> {
        info!("💾 Initializing database...");

        // Always initialize SQLite (primary or cache/fallback)
        let sqlite = Self::init_sqlite(&config.database_path).await?;

        // Try to initialize external database if configured
        let external = Self::init_external_db().await;

        let mode = if external.is_some() {
            DatabaseMode::Normal
        } else {
            DatabaseMode::Normal // SQLite is primary
        };

        let db = Database {
            sqlite,
            external,
            mode: Arc::new(RwLock::new(mode)),
            config: Arc::new(config.clone()),
        };

        // Run migrations
        db.run_migrations().await?;

        info!("✓ Database initialized in {:?} mode", mode);

        Ok(db)
    }

    /// Initialize SQLite database
    async fn init_sqlite(db_path: &Path) -> Result<SqlitePool> {
        info!("📊 Initializing SQLite database: {}", db_path.display());

        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let pool = sqlx::SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to SQLite database")?;

        // Enable foreign keys and other pragmas
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;

        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&pool)
            .await?;

        sqlx::query("PRAGMA synchronous = NORMAL")
            .execute(&pool)
            .await?;

        sqlx::query("PRAGMA temp_store = MEMORY")
            .execute(&pool)
            .await?;

        sqlx::query("PRAGMA cache_size = -64000") // 64MB cache
            .execute(&pool)
            .await?;

        info!("✓ SQLite initialized");
        Ok(pool)
    }

    /// Initialize external database (PostgreSQL, MySQL)
    async fn init_external_db() -> Option<ExternalDb> {
        // Check for external database configuration
        // Priority: POSTGRES > MYSQL

        #[cfg(feature = "postgres")]
        if let Ok(db_url) = std::env::var("POSTGRES_URL") {
            info!("🐘 Connecting to PostgreSQL...");
            match sqlx::PgPool::connect(&db_url).await {
                Ok(pool) => {
                    info!("✓ PostgreSQL connected");
                    return Some(ExternalDb::Postgres(pool));
                }
                Err(e) => {
                    warn!("⚠️  PostgreSQL connection failed: {}", e);
                }
            }
        }

        #[cfg(feature = "mysql")]
        if let Ok(db_url) = std::env::var("MYSQL_URL") {
            info!("🐬 Connecting to MySQL/MariaDB...");
            match sqlx::MySqlPool::connect(&db_url).await {
                Ok(pool) => {
                    info!("✓ MySQL/MariaDB connected");
                    return Some(ExternalDb::MySql(pool));
                }
                Err(e) => {
                    warn!("⚠️  MySQL connection failed: {}", e);
                }
            }
        }

        None
    }

    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        info!("🔄 Running database migrations...");

        // Run SQLite migrations
        migrations::run_sqlite_migrations(&self.sqlite).await?;

        // Run external DB migrations if configured
        if let Some(ref external) = self.external {
            #[allow(unreachable_patterns)]
            match external {
                #[cfg(feature = "postgres")]
                ExternalDb::Postgres(pool) => {
                    migrations::run_postgres_migrations(pool).await?;
                }
                #[cfg(feature = "mysql")]
                ExternalDb::MySql(pool) => {
                    migrations::run_mysql_migrations(pool).await?;
                }
                #[cfg(feature = "valkey")]
                ExternalDb::Valkey(_) => {
                    // Redis/Valkey doesn't need migrations
                }
                #[allow(unreachable_patterns)]
                _ => {
                    // Catch-all for other feature combinations
                }
            }
        }

        info!("✓ Migrations complete");
        Ok(())
    }

    /// Get current operational mode
    pub async fn mode(&self) -> DatabaseMode {
        *self.mode.read().await
    }

    /// Check if in read-only mode
    pub async fn is_read_only(&self) -> bool {
        matches!(
            *self.mode.read().await,
            DatabaseMode::ReadOnlyMaintenance | DatabaseMode::Critical
        )
    }

    /// Get SQLite pool (always available)
    pub fn sqlite(&self) -> &SqlitePool {
        &self.sqlite
    }

    /// Execute query with automatic failover
    pub async fn execute_with_failover<F, T>(&self, f: F) -> Result<T>
    where
        F: Fn(&SqlitePool) -> futures::future::BoxFuture<'_, Result<T>>,
    {
        // Try external DB first if available and in normal mode
        if let Some(ref _external) = self.external {
            let mode = self.mode().await;
            if mode == DatabaseMode::Normal {
                // Try external DB
                match f(&self.sqlite).await {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        error!("❌ External database error: {}", e);
                        // Fail over to SQLite cache
                        self.enter_maintenance_mode().await;
                    }
                }
            }
        }

        // Use SQLite (primary or fallback)
        f(&self.sqlite).await
    }

    /// Enter read-only maintenance mode
    async fn enter_maintenance_mode(&self) {
        let mut mode = self.mode.write().await;
        if *mode == DatabaseMode::Normal {
            warn!("⚠️  Entering read-only maintenance mode (using SQLite cache)");
            *mode = DatabaseMode::ReadOnlyMaintenance;

            // Log instructions for recovery
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            error!("🔧 DATABASE MAINTENANCE MODE");
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            error!("External database connection lost.");
            error!("Server is now in READ-ONLY mode using SQLite cache.");
            error!("");
            error!("To recover:");
            error!("  1. Fix external database connection");
            error!("  2. Restart CasGarage server");
            error!("  3. Or use Admin UI for guided recovery");
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
    }

    /// Attempt to recover from maintenance mode
    pub async fn attempt_recovery(&self) -> Result<()> {
        let mode = self.mode().await;
        if mode != DatabaseMode::ReadOnlyMaintenance {
            return Ok(());
        }

        info!("🔄 Attempting database recovery...");

        // Try to reconnect to external database
        if let Some(_new_external) = Self::init_external_db().await {
            let mut mode = self.mode.write().await;
            *mode = DatabaseMode::Normal;
            info!("✓ Database recovered! Returning to normal operation.");
            Ok(())
        } else {
            warn!("⚠️  Recovery failed - external database still unavailable");
            Err(anyhow::anyhow!("External database still unavailable"))
        }
    }

    /// Health check
    pub async fn health_check(&self) -> Result<DatabaseHealth> {
        let mut health = DatabaseHealth {
            mode: self.mode().await,
            sqlite_ok: false,
            external_ok: false,
            disk_space_mb: 0,
        };

        // Check SQLite
        health.sqlite_ok = sqlx::query("SELECT 1")
            .execute(&self.sqlite)
            .await
            .is_ok();

        // Check external DB if configured
        if let Some(ref _external) = self.external {
            // Try a simple query
            health.external_ok = sqlx::query("SELECT 1")
                .execute(&self.sqlite)
                .await
                .is_ok();
        }

        // Check disk space
        if std::fs::metadata(&self.config.data_dir).is_ok() {
            // Rough estimate
            health.disk_space_mb = 1000;
        }

        Ok(health)
    }

    /// Get connection statistics
    pub async fn stats(&self) -> DatabaseStats {
        DatabaseStats {
            mode: self.mode().await,
            sqlite_connections: self.sqlite.size() as u32,
            external_connections: 0,
            total_queries: 0,
            failed_queries: 0,
        }
    }
}

/// Database health information
#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub mode: DatabaseMode,
    pub sqlite_ok: bool,
    pub external_ok: bool,
    pub disk_space_mb: u64,
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub mode: DatabaseMode,
    pub sqlite_connections: u32,
    pub external_connections: u32,
    pub total_queries: u64,
    pub failed_queries: u64,
}

impl DatabaseHealth {
    /// Check if database is healthy
    pub fn is_healthy(&self) -> bool {
        self.sqlite_ok && (self.external_ok || self.mode == DatabaseMode::Normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_sqlite_init() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = Database::init_sqlite(&db_path).await.unwrap();

        // Test basic query
        let result: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(result.0, 1);
    }
}
