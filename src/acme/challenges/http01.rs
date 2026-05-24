//! HTTP-01 challenge implementation

use anyhow::Result;
use tracing::info;

/// Prepare HTTP-01 challenge
///
/// Creates /.well-known/acme-challenge/{token} endpoint
pub async fn prepare(token: &str, key_auth: &str) -> Result<()> {
    info!("Preparing HTTP-01 challenge: {}", token);

    // TODO: Store challenge response for serving via /.well-known/acme-challenge/{token}
    // For now, just log
    info!("HTTP-01 challenge prepared: token={}, key_auth={}", token, &key_auth[..20]);

    Ok(())
}

/// Cleanup HTTP-01 challenge
pub async fn cleanup(token: &str) -> Result<()> {
    info!("Cleaning up HTTP-01 challenge: {}", token);
    Ok(())
}
