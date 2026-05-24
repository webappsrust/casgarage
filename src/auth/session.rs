//! Session management
//!
//! Handles user sessions with secure cookies and session storage

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{User, UserRole};

/// Session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: String,

    /// User ID
    pub user_id: i64,

    /// Username
    pub username: String,

    /// User role
    pub role: UserRole,

    /// Created at (Unix timestamp)
    pub created_at: i64,

    /// Last accessed (Unix timestamp)
    pub last_accessed: i64,

    /// Expires at (Unix timestamp)
    pub expires_at: i64,
}

/// In-memory session store
/// In production, this would be backed by Redis/Valkey or database
#[derive(Clone)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionStore {
    /// Create new session store
    pub fn new() -> Self {
        SessionStore {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create new session for user
    pub async fn create_session(&self, user: &User) -> Result<Session> {
        let session_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let expiry = now + (24 * 60 * 60); // 24 hours

        let session = Session {
            id: session_id.clone(),
            user_id: user.id,
            username: user.username.clone(),
            role: user.role.clone(),
            created_at: now,
            last_accessed: now,
            expires_at: expiry,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session.clone());

        Ok(session)
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            let now = chrono::Utc::now().timestamp();

            // Check if expired
            if now > session.expires_at {
                sessions.remove(session_id);
                return None;
            }

            // Update last accessed
            session.last_accessed = now;

            Some(session.clone())
        } else {
            None
        }
    }

    /// Invalidate session
    pub async fn invalidate_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().await;
        let now = chrono::Utc::now().timestamp();

        sessions.retain(|_, session| session.expires_at > now);
    }

    /// Get session count
    pub async fn count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user() -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            email: Some("test@example.com".to_string()),
            role: UserRole::Admin,
            totp_secret: None,
            created_at: chrono::Utc::now().timestamp(),
            last_login: None,
        }
    }

    #[tokio::test]
    async fn test_session_creation() {
        let store = SessionStore::new();
        let user = create_test_user();

        let session = store.create_session(&user).await.unwrap();

        assert_eq!(session.user_id, user.id);
        assert_eq!(session.username, user.username);
        assert_eq!(session.role, user.role);
    }

    #[tokio::test]
    async fn test_session_retrieval() {
        let store = SessionStore::new();
        let user = create_test_user();

        let session = store.create_session(&user).await.unwrap();
        let session_id = session.id.clone();

        // Should be able to retrieve session
        let retrieved = store.get_session(&session_id).await;
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.user_id, user.id);
    }

    #[tokio::test]
    async fn test_session_invalidation() {
        let store = SessionStore::new();
        let user = create_test_user();

        let session = store.create_session(&user).await.unwrap();
        let session_id = session.id.clone();

        // Invalidate session
        store.invalidate_session(&session_id).await;

        // Should not be able to retrieve
        let retrieved = store.get_session(&session_id).await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_session_count() {
        let store = SessionStore::new();
        let user = create_test_user();

        assert_eq!(store.count().await, 0);

        store.create_session(&user).await.unwrap();
        assert_eq!(store.count().await, 1);

        store.create_session(&user).await.unwrap();
        assert_eq!(store.count().await, 2);
    }
}
