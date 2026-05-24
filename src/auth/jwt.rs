//! JWT token generation and validation

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

use super::UserRole;

/// JWT secret key (should be generated and stored in database in production)
/// For now, use a static key - will be replaced with database-stored key
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        // Generate random secret on first run
        use rand::Rng;
        let secret: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        secret
    })
});

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,

    /// Username
    pub username: String,

    /// User role
    pub role: UserRole,

    /// Issued at (Unix timestamp)
    pub iat: i64,

    /// Expiration time (Unix timestamp)
    pub exp: i64,

    /// Not before (Unix timestamp)
    pub nbf: i64,
}

/// Generate JWT token for user
pub fn generate_token(user_id: i64, username: &str, role: UserRole) -> Result<String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(24); // 24 hour expiry

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        role,
        iat: now.timestamp(),
        exp: expiration.timestamp(),
        nbf: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .context("Failed to generate JWT token")?;

    Ok(token)
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .context("Invalid JWT token")?;

    Ok(token_data.claims)
}

/// Refresh token (generate new token with updated expiry)
pub fn refresh_token(claims: &Claims) -> Result<String> {
    generate_token(
        claims.sub.parse()?,
        &claims.username,
        claims.role.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let token = generate_token(1, "testuser", UserRole::Admin).unwrap();

        // Token should not be empty
        assert!(!token.is_empty());

        // Should be able to validate and extract claims
        let claims = validate_token(&token).unwrap();
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, UserRole::Admin);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_refresh() {
        let token = generate_token(1, "testuser", UserRole::Operator).unwrap();
        let claims = validate_token(&token).unwrap();

        // Refresh token
        let new_token = refresh_token(&claims).unwrap();
        let new_claims = validate_token(&new_token).unwrap();

        // Username and role should match
        assert_eq!(new_claims.username, claims.username);
        assert_eq!(new_claims.role, claims.role);

        // Expiry should be updated (new token)
        assert!(new_claims.exp > claims.exp || new_claims.iat > claims.iat);
    }
}
