//! TLS-ALPN-01 challenge implementation

use anyhow::Result;
use tracing::info;

/// Prepare TLS-ALPN-01 challenge
///
/// Sets up special TLS certificate for validation
pub async fn prepare(domain: &str, _key_auth: &str) -> Result<()> {
    info!("Preparing TLS-ALPN-01 challenge for domain: {}", domain);

    // TODO: Generate special validation certificate
    // TODO: Configure TLS server to serve validation cert
    info!("TLS-ALPN-01 challenge prepared for: {}", domain);

    Ok(())
}

/// Cleanup TLS-ALPN-01 challenge
pub async fn cleanup(domain: &str) -> Result<()> {
    info!("Cleaning up TLS-ALPN-01 challenge for domain: {}", domain);
    Ok(())
}
