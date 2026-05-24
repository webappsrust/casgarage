//! Configuration management module
//!
//! Handles CLI arguments, environment variables, and runtime configuration.
//! All configuration is database-driven after initial startup.

use anyhow::{Context, Result};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

use crate::cli::Cli;

/// Main application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Port configuration (single or HTTP,HTTPS)
    pub ports: PortConfig,

    /// Server address (resolved, never 0.0.0.0/127.0.0.1/localhost)
    pub server_address: ServerAddress,

    /// Data directory
    pub data_dir: PathBuf,

    /// Config directory (for SSL certs, runtime files)
    pub config_dir: PathBuf,

    /// Log directory
    pub log_dir: PathBuf,

    /// Database path (SQLite)
    pub database_path: PathBuf,

    /// Blocks storage path
    pub blocks_dir: PathBuf,
}

/// Port configuration
#[derive(Debug, Clone)]
pub enum PortConfig {
    /// Single HTTP port
    Single(u16),
    /// HTTP and HTTPS ports
    Dual { http: u16, https: u16 },
}

/// Server address with hostname resolution
#[derive(Debug, Clone)]
pub struct ServerAddress {
    /// Resolved IP address
    pub ip: IpAddr,
    /// Hostname (FQDN or IP as string, never localhost/127.0.0.1/0.0.0.0)
    pub display: String,
}

impl Config {
    /// Create configuration from CLI arguments
    pub fn from_cli(cli: Cli) -> Result<Self> {
        info!("🔧 Initializing configuration...");

        // Parse ports
        let ports = Self::parse_ports(cli.port)?;

        // Resolve server address
        let server_address = Self::resolve_address(cli.address)?;

        // Resolve directories
        let data_dir = Self::resolve_directory(
            cli.datadir,
            std::env::var("DATA_DIR").ok(),
            Self::default_data_dir(),
            "data",
        )?;

        let config_dir = Self::resolve_directory(
            cli.configdir,
            std::env::var("CONFIG_DIR").ok(),
            Self::default_config_dir(),
            "config",
        )?;

        let log_dir = Self::resolve_directory(
            cli.logdir,
            std::env::var("LOG_DIR").ok(),
            Self::default_log_dir(),
            "logs",
        )?;

        // Ensure directories exist
        std::fs::create_dir_all(&data_dir)
            .context("Failed to create data directory")?;
        std::fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
        std::fs::create_dir_all(&log_dir)
            .context("Failed to create log directory")?;

        let database_path = data_dir.join("db").join("casgarage.db");
        std::fs::create_dir_all(database_path.parent().unwrap())
            .context("Failed to create database directory")?;

        let blocks_dir = data_dir.join("blocks");
        std::fs::create_dir_all(&blocks_dir)
            .context("Failed to create blocks directory")?;

        let config = Config {
            ports,
            server_address,
            data_dir,
            config_dir,
            log_dir,
            database_path,
            blocks_dir,
        };

        config.log_configuration();
        config.validate()?;

        Ok(config)
    }

    /// Parse port configuration from string
    fn parse_ports(port_str: Option<String>) -> Result<PortConfig> {
        let port_str = port_str.or_else(|| std::env::var("PORT").ok());

        match port_str {
            Some(ports) => {
                if ports.contains(',') {
                    // Dual port: "80,443"
                    let parts: Vec<&str> = ports.split(',').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Port must be single port or HTTP,HTTPS format (e.g., '80,443')");
                    }

                    let http = parts[0].trim().parse::<u16>()
                        .context("Invalid HTTP port")?;
                    let https = parts[1].trim().parse::<u16>()
                        .context("Invalid HTTPS port")?;

                    Ok(PortConfig::Dual { http, https })
                } else {
                    // Single port
                    let port = ports.parse::<u16>()
                        .context("Invalid port number")?;
                    Ok(PortConfig::Single(port))
                }
            }
            None => {
                // Find random unused port in 64xxx range
                let port = Self::find_unused_port()?;
                info!("🔌 No port specified, selected random port: {}", port);
                Ok(PortConfig::Single(port))
            }
        }
    }

    /// Find an unused port in 64000-65000 range
    fn find_unused_port() -> Result<u16> {
        use std::net::TcpListener;

        for port in 64000..=65000 {
            if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
                drop(listener);
                return Ok(port);
            }
        }

        anyhow::bail!("Could not find unused port in 64000-65000 range");
    }

    /// Resolve server address (never show 0.0.0.0/127.0.0.1/localhost)
    fn resolve_address(address: Option<String>) -> Result<ServerAddress> {
        let addr_str = address
            .or_else(|| std::env::var("SERVER_ADDRESS").ok())
            .unwrap_or_else(|| "0.0.0.0".to_string());

        let ip: IpAddr = addr_str.parse()
            .context("Invalid server address")?;

        // Resolve display hostname
        let display = if ip.is_unspecified() || ip.is_loopback() {
            // Try to get actual IP
            Self::get_primary_ip()
                .unwrap_or_else(|| "localhost".to_string())
        } else {
            addr_str
        };

        Ok(ServerAddress { ip, display })
    }

    /// Get primary IP address of the system
    fn get_primary_ip() -> Option<String> {
        // Try to get primary interface IP
        if let Ok(interfaces) = if_addrs::get_if_addrs() {
            for iface in interfaces {
                if !iface.is_loopback() {
                    match iface.ip() {
                        IpAddr::V4(ipv4) if !ipv4.is_loopback() && !ipv4.is_link_local() => {
                            return Some(ipv4.to_string());
                        }
                        IpAddr::V6(ipv6) if !ipv6.is_loopback() => {
                            return Some(format!("[{}]", ipv6));
                        }
                        _ => continue,
                    }
                }
            }
        }

        // Fallback to hostname
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
    }

    /// Resolve directory path with fallbacks
    fn resolve_directory(
        cli_value: Option<PathBuf>,
        env_value: Option<String>,
        default: PathBuf,
        name: &str,
    ) -> Result<PathBuf> {
        let path = cli_value
            .or_else(|| env_value.map(PathBuf::from))
            .unwrap_or(default);

        let absolute = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()?.join(path)
        };

        debug!("📁 Resolved {} directory: {}", name, absolute.display());
        Ok(absolute)
    }

    /// Default data directory
    fn default_data_dir() -> PathBuf {
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
            PathBuf::from("/var/lib/casgarage")
        } else if cfg!(target_os = "macos") {
            PathBuf::from("/usr/local/var/casgarage")
        } else if cfg!(target_os = "windows") {
            PathBuf::from(r"C:\ProgramData\CasGarage\data")
        } else {
            PathBuf::from("./data")
        }
    }

    /// Default config directory
    fn default_config_dir() -> PathBuf {
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
            PathBuf::from("/etc/casgarage")
        } else if cfg!(target_os = "macos") {
            PathBuf::from("/usr/local/etc/casgarage")
        } else if cfg!(target_os = "windows") {
            PathBuf::from(r"C:\ProgramData\CasGarage\config")
        } else {
            PathBuf::from("./config")
        }
    }

    /// Default log directory
    fn default_log_dir() -> PathBuf {
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
            PathBuf::from("/var/log/casgarage")
        } else if cfg!(target_os = "macos") {
            PathBuf::from("/usr/local/var/log/casgarage")
        } else if cfg!(target_os = "windows") {
            PathBuf::from(r"C:\ProgramData\CasGarage\logs")
        } else {
            PathBuf::from("./logs")
        }
    }

    /// Get bind addresses for HTTP server
    pub fn http_bind_address(&self) -> SocketAddr {
        let port = match self.ports {
            PortConfig::Single(p) => p,
            PortConfig::Dual { http, .. } => http,
        };

        SocketAddr::new(self.server_address.ip, port)
    }

    /// Get bind address for HTTPS server (if configured)
    pub fn https_bind_address(&self) -> Option<SocketAddr> {
        match self.ports {
            PortConfig::Single(_) => None,
            PortConfig::Dual { https, .. } => {
                Some(SocketAddr::new(self.server_address.ip, https))
            }
        }
    }

    /// Check if running on privileged ports (80,443)
    pub fn is_privileged_ports(&self) -> bool {
        match self.ports {
            PortConfig::Single(p) => p == 80 || p == 443,
            PortConfig::Dual { http, https } => http == 80 && https == 443,
        }
    }

    /// Get SSL certificate directory
    pub fn ssl_cert_dir(&self) -> PathBuf {
        self.config_dir.join("ssl").join("certs")
    }

    /// Check for existing Let's Encrypt certificates
    pub fn check_existing_letsencrypt(&self) -> Option<PathBuf> {
        let le_path = PathBuf::from("/etc/letsencrypt/live");
        if le_path.exists() {
            // Check for certificates
            if let Ok(entries) = std::fs::read_dir(&le_path) {
                for entry in entries.flatten() {
                    let fullchain = entry.path().join("fullchain.pem");
                    let privkey = entry.path().join("privkey.pem");
                    if fullchain.exists() && privkey.exists() {
                        info!("📜 Found existing Let's Encrypt certificate: {}", entry.path().display());
                        return Some(entry.path());
                    }
                }
            }
        }
        None
    }

    /// Validate configuration
    fn validate(&self) -> Result<()> {
        // Check write permissions
        let test_file = self.data_dir.join(".write_test");
        std::fs::write(&test_file, "test")
            .context("No write permission to data directory")?;
        std::fs::remove_file(test_file).ok();

        let test_file = self.config_dir.join(".write_test");
        std::fs::write(&test_file, "test")
            .context("No write permission to config directory")?;
        std::fs::remove_file(test_file).ok();

        let test_file = self.log_dir.join(".write_test");
        std::fs::write(&test_file, "test")
            .context("No write permission to log directory")?;
        std::fs::remove_file(test_file).ok();

        Ok(())
    }

    /// Log configuration details
    fn log_configuration(&self) {
        info!("📋 Configuration initialized:");
        info!("  ├─ Ports: {}", self.format_ports());
        info!("  ├─ Address: {} ({})", self.server_address.display, self.server_address.ip);
        info!("  ├─ Data: {}", self.data_dir.display());
        info!("  ├─ Config: {}", self.config_dir.display());
        info!("  ├─ Logs: {}", self.log_dir.display());
        info!("  └─ Database: {}", self.database_path.display());

        if self.is_privileged_ports() {
            info!("🔐 Running on privileged ports - Let's Encrypt will be enabled");
        }
    }

    /// Format ports for display
    fn format_ports(&self) -> String {
        match self.ports {
            PortConfig::Single(p) => format!("{} (HTTP)", p),
            PortConfig::Dual { http, https } => format!("{} (HTTP), {} (HTTPS)", http, https),
        }
    }

    /// Get display URL for users
    pub fn display_url(&self) -> String {
        let scheme = if self.https_bind_address().is_some() { "https" } else { "http" };
        let port = match self.ports {
            PortConfig::Single(p) => p,
            PortConfig::Dual { https, .. } => https,
        };

        // Don't show port if standard
        if (scheme == "http" && port == 80) || (scheme == "https" && port == 443) {
            format!("{}://{}", scheme, self.server_address.display)
        } else {
            format!("{}://{}:{}", scheme, self.server_address.display, port)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_port() {
        let config = Config::parse_ports(Some("8080".to_string())).unwrap();
        match config {
            PortConfig::Single(p) => assert_eq!(p, 8080),
            _ => panic!("Expected single port"),
        }
    }

    #[test]
    fn test_parse_dual_ports() {
        let config = Config::parse_ports(Some("80,443".to_string())).unwrap();
        match config {
            PortConfig::Dual { http, https } => {
                assert_eq!(http, 80);
                assert_eq!(https, 443);
            }
            _ => panic!("Expected dual ports"),
        }
    }

    #[test]
    fn test_find_unused_port() {
        let port = Config::find_unused_port().unwrap();
        assert!(port >= 64000 && port <= 65000);
    }
}
