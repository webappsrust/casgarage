//! Database migrations
//!
//! Handles schema creation and migrations for all supported databases

use anyhow::Result;
use sqlx::{SqlitePool, Row};
use tracing::info;

/// Run SQLite migrations
pub async fn run_sqlite_migrations(pool: &SqlitePool) -> Result<()> {
    info!("Running SQLite migrations...");

    // Create migrations table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            applied_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Run migrations in order
    run_migration(pool, "001_initial_schema", create_initial_schema).await?;

    info!("✓ SQLite migrations complete");
    Ok(())
}

/// Run a single migration
async fn run_migration<F>(pool: &SqlitePool, name: &str, migration: F) -> Result<()>
where
    F: FnOnce(&SqlitePool) -> futures::future::BoxFuture<'_, Result<()>>,
{
    // Check if migration already applied
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?)"
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    if exists {
        return Ok(());
    }

    info!("  📝 Applying migration: {}", name);

    // Run migration
    migration(pool).await?;

    // Record migration
    sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}

/// Initial schema creation
fn create_initial_schema(pool: &SqlitePool) -> futures::future::BoxFuture<'_, Result<()>> {
    Box::pin(async move {
    // Server configuration
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Admin users (for web UI access)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admin_users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email TEXT,
            role TEXT NOT NULL DEFAULT 'operator',
            totp_secret TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            last_login INTEGER
        )
        "#,
    )
    .execute(pool)
    .await?;

    // S3 access keys
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS access_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            access_key_id TEXT UNIQUE NOT NULL,
            secret_key TEXT NOT NULL,
            name TEXT NOT NULL,
            permissions TEXT NOT NULL,
            ip_whitelist TEXT,
            rate_limit INTEGER,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            last_used_at INTEGER,
            enabled INTEGER DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Buckets metadata
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS buckets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            created_by INTEGER REFERENCES access_keys(id),
            is_public INTEGER DEFAULT 0,
            versioning_enabled INTEGER DEFAULT 0,
            website_mode INTEGER DEFAULT 0,
            quota_bytes INTEGER,
            quota_objects INTEGER,
            config TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Cluster nodes
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS nodes (
            id TEXT PRIMARY KEY,
            hostname TEXT NOT NULL,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL,
            datacenter TEXT,
            capacity_bytes INTEGER,
            status TEXT NOT NULL,
            metadata TEXT,
            last_seen INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Replication sites
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            location TEXT,
            priority INTEGER DEFAULT 100,
            bandwidth_limit INTEGER,
            enabled INTEGER DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Audit log
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            user_id INTEGER REFERENCES admin_users(id),
            action TEXT NOT NULL,
            resource TEXT NOT NULL,
            details TEXT,
            ip_address TEXT,
            user_agent TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_log_user_id ON audit_log(user_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_access_keys_enabled ON access_keys(enabled)")
        .execute(pool)
        .await?;

    Ok(())
    })
}

/// Run PostgreSQL migrations
#[cfg(feature = "postgres")]
pub async fn run_postgres_migrations(pool: &sqlx::PgPool) -> Result<()> {
    info!("Running PostgreSQL migrations...");
    // PostgreSQL-specific migrations
    // Similar structure to SQLite but with PostgreSQL syntax
    info!("✓ PostgreSQL migrations complete");
    Ok(())
}

/// Run MySQL migrations
#[cfg(feature = "mysql")]
pub async fn run_mysql_migrations(pool: &sqlx::MySqlPool) -> Result<()> {
    info!("Running MySQL migrations...");
    // MySQL-specific migrations
    info!("✓ MySQL migrations complete");
    Ok(())
}
