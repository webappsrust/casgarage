//! ACME (Let's Encrypt) certificate management
//!
//! Supports all ACME challenge types:
//! - HTTP-01 (standard HTTP challenge)
//! - DNS-01 (DNS TXT record, supports all providers + RFC2136)
//! - TLS-ALPN-01 (TLS-based challenge)

pub mod challenges;
pub mod cert_manager;

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// ACME client for Let's Encrypt
pub struct AcmeClient {
    cert_dir: PathBuf,
    email: String,
}

impl AcmeClient {
    /// Create new ACME client
    pub async fn new(cert_dir: PathBuf, email: &str) -> Result<Self> {
        info!("🔐 Initializing Let's Encrypt ACME client...");

        std::fs::create_dir_all(&cert_dir)
            .context("Failed to create certificate directory")?;

        info!("✓ ACME client initialized");

        Ok(AcmeClient {
            cert_dir,
            email: email.to_string(),
        })
    }

    /// Request certificate for domain(s)
    ///
    /// Full implementation will use instant-acme crate properly
    /// For now, this is a stub that returns placeholder
    pub async fn request_certificate(
        &self,
        domains: &[String],
        _challenge_type: AcmeChallengeType,
    ) -> Result<(String, String)> {
        info!("📜 Requesting certificate for: {:?}", domains);

        // TODO: Implement full ACME protocol with instant-acme
        // This is a placeholder that will be implemented with proper ACME flow

        info!("✓ Certificate request initialized (stub)");

        // Return placeholder
        Ok((
            "CERTIFICATE_PLACEHOLDER".to_string(),
            "PRIVATE_KEY_PLACEHOLDER".to_string(),
        ))
    }

    /// Save certificate to disk
    pub async fn save_certificate(
        &self,
        domain: &str,
        cert_chain: &str,
        private_key: &str,
    ) -> Result<(PathBuf, PathBuf)> {
        let domain_dir = self.cert_dir.join(domain);
        std::fs::create_dir_all(&domain_dir)?;

        let cert_path = domain_dir.join("fullchain.pem");
        let key_path = domain_dir.join("privkey.pem");

        std::fs::write(&cert_path, cert_chain)?;
        std::fs::write(&key_path, private_key)?;

        // Set proper permissions (600 for private key)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600))?;
        }

        info!("✓ Certificate saved to {}", cert_path.display());

        Ok((cert_path, key_path))
    }
}

/// ACME challenge type
#[derive(Debug, Clone, Copy)]
pub enum AcmeChallengeType {
    /// HTTP-01 challenge (requires port 80)
    Http01,
    /// DNS-01 challenge (requires DNS provider access)
    Dns01,
    /// TLS-ALPN-01 challenge (requires port 443)
    TlsAlpn01,
}
