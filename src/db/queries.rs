//! Database query functions
//!
//! High-level query functions for all database operations

use anyhow::Result;
use sqlx::SqlitePool;

use super::models::*;

// ============================================================================
// Configuration Queries
// ============================================================================

/// Get configuration value by key
pub async fn get_config(pool: &SqlitePool, key: &str) -> Result<Option<String>> {
    let value: Option<String> = sqlx::query_scalar(
        "SELECT value FROM config WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(value)
}

/// Set configuration value
pub async fn set_config(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?, ?)"
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// Bucket Queries
// ============================================================================

/// List all buckets
pub async fn list_buckets(pool: &SqlitePool) -> Result<Vec<Bucket>> {
    let buckets = sqlx::query_as::<_, Bucket>(
        "SELECT id, name, created_at, created_by, is_public, versioning_enabled,
                website_mode, quota_bytes, quota_objects, config
         FROM buckets
         ORDER BY name"
    )
    .fetch_all(pool)
    .await?;

    Ok(buckets)
}

/// Get bucket by name
pub async fn get_bucket(pool: &SqlitePool, name: &str) -> Result<Option<Bucket>> {
    let bucket = sqlx::query_as::<_, Bucket>(
        "SELECT id, name, created_at, created_by, is_public, versioning_enabled,
                website_mode, quota_bytes, quota_objects, config
         FROM buckets
         WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(bucket)
}

/// Create bucket
pub async fn create_bucket(
    pool: &SqlitePool,
    name: &str,
    created_by: Option<i64>,
    is_public: bool,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT INTO buckets (name, created_by, is_public) VALUES (?, ?, ?)"
    )
    .bind(name)
    .bind(created_by)
    .bind(is_public)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Delete bucket
pub async fn delete_bucket(pool: &SqlitePool, name: &str) -> Result<()> {
    sqlx::query("DELETE FROM buckets WHERE name = ?")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}

// ============================================================================
// Access Key Queries
// ============================================================================

/// List all access keys
pub async fn list_access_keys(pool: &SqlitePool) -> Result<Vec<AccessKey>> {
    let keys = sqlx::query_as::<_, AccessKey>(
        "SELECT id, access_key_id, secret_key, name, permissions, ip_whitelist,
                rate_limit, created_at, last_used_at, enabled
         FROM access_keys
         WHERE enabled = 1
         ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(keys)
}

/// Get access key by ID
pub async fn get_access_key(pool: &SqlitePool, access_key_id: &str) -> Result<Option<AccessKey>> {
    let key = sqlx::query_as::<_, AccessKey>(
        "SELECT id, access_key_id, secret_key, name, permissions, ip_whitelist,
                rate_limit, created_at, last_used_at, enabled
         FROM access_keys
         WHERE access_key_id = ?"
    )
    .bind(access_key_id)
    .fetch_optional(pool)
    .await?;

    Ok(key)
}

/// Create access key
pub async fn create_access_key(
    pool: &SqlitePool,
    access_key_id: &str,
    secret_key: &str,
    name: &str,
    permissions: &str,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT INTO access_keys (access_key_id, secret_key, name, permissions)
         VALUES (?, ?, ?, ?)"
    )
    .bind(access_key_id)
    .bind(secret_key)
    .bind(name)
    .bind(permissions)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update last used timestamp for access key
pub async fn update_key_last_used(pool: &SqlitePool, access_key_id: &str) -> Result<()> {
    sqlx::query(
        "UPDATE access_keys SET last_used_at = strftime('%s', 'now') WHERE access_key_id = ?"
    )
    .bind(access_key_id)
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// Node/Cluster Queries
// ============================================================================

/// List all nodes
pub async fn list_nodes(pool: &SqlitePool) -> Result<Vec<Node>> {
    let nodes = sqlx::query_as::<_, Node>(
        "SELECT id, hostname, ip_address, port, datacenter, capacity_bytes,
                status, metadata, last_seen
         FROM nodes
         ORDER BY hostname"
    )
    .fetch_all(pool)
    .await?;

    Ok(nodes)
}

/// Get node by ID
pub async fn get_node(pool: &SqlitePool, node_id: &str) -> Result<Option<Node>> {
    let node = sqlx::query_as::<_, Node>(
        "SELECT id, hostname, ip_address, port, datacenter, capacity_bytes,
                status, metadata, last_seen
         FROM nodes
         WHERE id = ?"
    )
    .bind(node_id)
    .fetch_optional(pool)
    .await?;

    Ok(node)
}

/// Update node last seen timestamp
pub async fn update_node_heartbeat(pool: &SqlitePool, node_id: &str) -> Result<()> {
    sqlx::query(
        "UPDATE nodes SET last_seen = strftime('%s', 'now') WHERE id = ?"
    )
    .bind(node_id)
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// Site/Datacenter Queries
// ============================================================================

/// List all sites
pub async fn list_sites(pool: &SqlitePool) -> Result<Vec<Site>> {
    let sites = sqlx::query_as::<_, Site>(
        "SELECT id, name, location, priority, bandwidth_limit, enabled
         FROM sites
         WHERE enabled = 1
         ORDER BY priority DESC, name"
    )
    .fetch_all(pool)
    .await?;

    Ok(sites)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn setup_test_db() -> (SqlitePool, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let pool = SqlitePool::connect(&db_url).await.unwrap();

        // Run migrations
        crate::db::migrations::run_sqlite_migrations(&pool)
            .await
            .unwrap();

        (pool, temp_dir)
    }

    #[tokio::test]
    async fn test_config_queries() {
        let (pool, _temp) = setup_test_db().await;

        // Set and get config
        set_config(&pool, "test_key", "test_value").await.unwrap();
        let value = get_config(&pool, "test_key").await.unwrap();

        assert_eq!(value, Some("test_value".to_string()));
    }

    #[tokio::test]
    async fn test_bucket_queries() {
        let (pool, _temp) = setup_test_db().await;

        // Create bucket
        let bucket_id = create_bucket(&pool, "test-bucket", None, false)
            .await
            .unwrap();
        assert!(bucket_id > 0);

        // Get bucket
        let bucket = get_bucket(&pool, "test-bucket").await.unwrap();
        assert!(bucket.is_some());
        assert_eq!(bucket.unwrap().name, "test-bucket");

        // List buckets
        let buckets = list_buckets(&pool).await.unwrap();
        assert_eq!(buckets.len(), 1);
    }
}
