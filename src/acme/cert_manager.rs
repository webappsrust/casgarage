//! Certificate lifecycle management
//!
//! Handles certificate renewal, validation, and storage

use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Certificate manager
pub struct CertManager {
    cert_dir: PathBuf,
}

impl CertManager {
    /// Create new certificate manager
    pub fn new(cert_dir: PathBuf) -> Self {
        CertManager { cert_dir }
    }

    /// Check if certificate exists for domain
    pub fn has_certificate(&self, domain: &str) -> bool {
        let cert_path = self.cert_dir.join(domain).join("fullchain.pem");
        let key_path = self.cert_dir.join(domain).join("privkey.pem");

        cert_path.exists() && key_path.exists()
    }

    /// Check if certificate needs renewal (< 30 days remaining)
    pub async fn needs_renewal(&self, domain: &str) -> Result<bool> {
        if !self.has_certificate(domain) {
            return Ok(true);
        }

        // TODO: Parse certificate and check expiry date
        // For now, return false
        Ok(false)
    }

    /// Get certificate paths
    pub fn get_cert_paths(&self, domain: &str) -> (PathBuf, PathBuf) {
        let cert_path = self.cert_dir.join(domain).join("fullchain.pem");
        let key_path = self.cert_dir.join(domain).join("privkey.pem");
        (cert_path, key_path)
    }

    /// List all managed certificates
    pub fn list_certificates(&self) -> Result<Vec<String>> {
        let mut domains = Vec::new();

        if !self.cert_dir.exists() {
            return Ok(domains);
        }

        for entry in std::fs::read_dir(&self.cert_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(domain) = entry.file_name().to_str() {
                    if self.has_certificate(domain) {
                        domains.push(domain.to_string());
                    }
                }
            }
        }

        Ok(domains)
    }
}
