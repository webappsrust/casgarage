/// Database schema definitions

pub const CREATE_CONFIG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
)"#;

pub const CREATE_ACCESS_KEYS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS access_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    access_key_id TEXT UNIQUE NOT NULL,
    secret_key TEXT NOT NULL,
    name TEXT NOT NULL,
    permissions TEXT NOT NULL,
    ip_whitelist TEXT,
    rate_limit INTEGER,
    created_at INTEGER NOT NULL,
    last_used_at INTEGER,
    enabled BOOLEAN DEFAULT 1
)"#;

pub const CREATE_BUCKETS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS buckets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    created_at INTEGER NOT NULL,
    created_by INTEGER REFERENCES access_keys(id),
    is_public BOOLEAN DEFAULT 0,
    versioning_enabled BOOLEAN DEFAULT 0,
    website_mode BOOLEAN DEFAULT 0,
    quota_bytes INTEGER,
    quota_objects INTEGER,
    config TEXT
)"#;

pub const CREATE_NODES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    hostname TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    datacenter TEXT,
    capacity_bytes INTEGER,
    status TEXT NOT NULL,
    metadata TEXT,
    last_seen INTEGER NOT NULL
)"#;

pub const CREATE_SITES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS sites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    location TEXT,
    priority INTEGER DEFAULT 100,
    bandwidth_limit INTEGER,
    enabled BOOLEAN DEFAULT 1
)"#;

pub const CREATE_ADMIN_USERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS admin_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT,
    role TEXT NOT NULL,
    totp_secret TEXT,
    created_at INTEGER NOT NULL,
    last_login INTEGER
)"#;

pub const CREATE_AUDIT_LOG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    user_id INTEGER REFERENCES admin_users(id),
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    details TEXT,
    ip_address TEXT,
    user_agent TEXT
)"#;

pub const CREATE_BACKUPS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    size_bytes INTEGER,
    status TEXT NOT NULL,
    destination TEXT NOT NULL,
    metadata TEXT
)"#;

pub const CREATE_ALERTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS alerts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    condition TEXT NOT NULL,
    enabled BOOLEAN DEFAULT 1,
    notification_channels TEXT,
    last_triggered INTEGER
)"#;
