//! Database models
//!
//! Rust structs representing database tables

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Server configuration key-value pair
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub updated_at: i64,
}

/// Admin user (for web UI access)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub role: String,
    pub totp_secret: Option<String>,
    pub created_at: i64,
    pub last_login: Option<i64>,
}

/// S3 Access key
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessKey {
    pub id: i64,
    pub access_key_id: String,
    #[serde(skip_serializing)]
    pub secret_key: String,
    pub name: String,
    pub permissions: String, // JSON
    pub ip_whitelist: Option<String>, // JSON array
    pub rate_limit: Option<i64>,
    pub created_at: i64,
    pub last_used_at: Option<i64>,
    pub enabled: bool,
}

/// S3 Bucket metadata
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bucket {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub created_by: Option<i64>,
    pub is_public: bool,
    pub versioning_enabled: bool,
    pub website_mode: bool,
    pub quota_bytes: Option<i64>,
    pub quota_objects: Option<i64>,
    pub config: Option<String>, // JSON
}

/// Cluster node
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub port: i64,
    pub datacenter: Option<String>,
    pub capacity_bytes: Option<i64>,
    pub status: String,
    pub metadata: Option<String>, // JSON
    pub last_seen: i64,
}

/// Replication site
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub location: Option<String>,
    pub priority: i64,
    pub bandwidth_limit: Option<i64>,
    pub enabled: bool,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub resource: String,
    pub details: Option<String>, // JSON
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
