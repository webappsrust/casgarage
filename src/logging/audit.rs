//! Audit logging for administrative actions

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub user_id: Option<i64>,
    pub action: String,
    pub resource: String,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Log an audit entry
pub async fn log_audit(pool: &SqlitePool, entry: AuditEntry) -> Result<()> {
    let details_json = entry.details.map(|d| d.to_string());

    sqlx::query(
        "INSERT INTO audit_log (user_id, action, resource, details, ip_address, user_agent)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(entry.user_id)
    .bind(&entry.action)
    .bind(&entry.resource)
    .bind(details_json)
    .bind(entry.ip_address)
    .bind(entry.user_agent)
    .execute(pool)
    .await?;

    info!(
        action = %entry.action,
        resource = %entry.resource,
        user_id = ?entry.user_id,
        "Audit log entry created"
    );

    Ok(())
}

/// Query audit logs
pub async fn query_audit_logs(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::db::models::AuditLogEntry>> {
    let entries = sqlx::query_as::<_, crate::db::models::AuditLogEntry>(
        "SELECT id, timestamp, user_id, action, resource, details, ip_address, user_agent
         FROM audit_log
         ORDER BY timestamp DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(entries)
}
