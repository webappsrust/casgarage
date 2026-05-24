//! Authentication and authorization module
//!
//! Handles user authentication, password hashing, JWT tokens, and sessions

pub mod first_user;
pub mod jwt;
pub mod password;
pub mod session;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{debug, info};

pub use jwt::{generate_token, validate_token, Claims};
pub use password::{hash_password, verify_password};

/// User role for RBAC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Full administrator access
    Admin,
    /// Can manage resources but not server settings
    Operator,
    /// Read-only access
    Viewer,
    /// Guest/anonymous user
    Guest,
}

impl UserRole {
    /// Check if role has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    /// Check if role can modify resources
    pub fn can_write(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Operator)
    }

    /// Check if role can view resources
    pub fn can_read(&self) -> bool {
        !matches!(self, UserRole::Guest)
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub totp_secret: Option<String>,
    pub created_at: i64,
    pub last_login: Option<i64>,
}

/// Check if any admin users exist
pub async fn has_admin_users(pool: &SqlitePool) -> Result<bool> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM admin_users WHERE role = 'admin'"
    )
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}

/// Get user by username
pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, email, role, totp_secret, created_at, last_login
         FROM admin_users
         WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}

/// Get user by ID
pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, email, role, totp_secret, created_at, last_login
         FROM admin_users
         WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}

/// Create new user
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
    email: Option<&str>,
    role: UserRole,
) -> Result<User> {
    info!("Creating user: {} with role: {:?}", username, role);

    // Hash password
    let password_hash = password::hash_password(password)?;

    // Insert user
    let result = sqlx::query(
        "INSERT INTO admin_users (username, password_hash, email, role) VALUES (?, ?, ?, ?)"
    )
    .bind(username)
    .bind(&password_hash)
    .bind(email)
    .bind(format!("{:?}", role).to_lowercase())
    .execute(pool)
    .await?;

    let user_id = result.last_insert_rowid();

    // Fetch created user
    let user = get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch created user"))?;

    debug!("User created successfully: {}", username);

    Ok(user)
}

/// Authenticate user with username and password
pub async fn authenticate_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<Option<User>> {
    debug!("Authenticating user: {}", username);

    // Get user with password hash
    let row = sqlx::query_as::<_, UserWithPasswordRow>(
        "SELECT id, username, password_hash, email, role, totp_secret, created_at, last_login
         FROM admin_users
         WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(user_row) => {
            // Verify password
            if password::verify_password(password, &user_row.password_hash)? {
                // Update last login
                sqlx::query("UPDATE admin_users SET last_login = strftime('%s', 'now') WHERE id = ?")
                    .bind(user_row.id)
                    .execute(pool)
                    .await?;

                debug!("Authentication successful for user: {}", username);

                Ok(Some(User {
                    id: user_row.id,
                    username: user_row.username,
                    email: user_row.email,
                    role: parse_role(&user_row.role),
                    totp_secret: user_row.totp_secret,
                    created_at: user_row.created_at,
                    last_login: user_row.last_login,
                }))
            } else {
                debug!("Authentication failed - invalid password for user: {}", username);
                Ok(None)
            }
        }
        None => {
            debug!("Authentication failed - user not found: {}", username);
            Ok(None)
        }
    }
}

/// Parse role from string
fn parse_role(role_str: &str) -> UserRole {
    match role_str.to_lowercase().as_str() {
        "admin" => UserRole::Admin,
        "operator" => UserRole::Operator,
        "viewer" => UserRole::Viewer,
        "guest" => UserRole::Guest,
        _ => UserRole::Viewer, // Default to viewer for unknown roles
    }
}

// Database row structures

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: i64,
    username: String,
    email: Option<String>,
    role: String,
    totp_secret: Option<String>,
    created_at: i64,
    last_login: Option<i64>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            username: row.username,
            email: row.email,
            role: parse_role(&row.role),
            totp_secret: row.totp_secret,
            created_at: row.created_at,
            last_login: row.last_login,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct UserWithPasswordRow {
    id: i64,
    username: String,
    password_hash: String,
    email: Option<String>,
    role: String,
    totp_secret: Option<String>,
    created_at: i64,
    last_login: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_parsing() {
        assert_eq!(parse_role("admin"), UserRole::Admin);
        assert_eq!(parse_role("ADMIN"), UserRole::Admin);
        assert_eq!(parse_role("operator"), UserRole::Operator);
        assert_eq!(parse_role("viewer"), UserRole::Viewer);
        assert_eq!(parse_role("unknown"), UserRole::Viewer);
    }

    #[test]
    fn test_role_permissions() {
        assert!(UserRole::Admin.is_admin());
        assert!(UserRole::Admin.can_write());
        assert!(UserRole::Admin.can_read());

        assert!(!UserRole::Operator.is_admin());
        assert!(UserRole::Operator.can_write());
        assert!(UserRole::Operator.can_read());

        assert!(!UserRole::Viewer.can_write());
        assert!(UserRole::Viewer.can_read());

        assert!(!UserRole::Guest.can_read());
    }
}
