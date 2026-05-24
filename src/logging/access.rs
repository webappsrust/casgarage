//! Access log implementation
//!
//! Apache Common Log Format (default, configurable via admin UI)

use chrono::Utc;
use std::net::IpAddr;

/// Access log format
#[derive(Debug, Clone, Copy)]
pub enum AccessLogFormat {
    /// Apache Common Log Format
    /// Format: %h %l %u %t "%r" %>s %b
    ApacheCommon,

    /// Apache Combined Log Format
    /// Format: %h %l %u %t "%r" %>s %b "%{Referer}i" "%{User-agent}i"
    ApacheCombined,

    /// JSON format
    Json,
}

/// Access log entry
#[derive(Debug, Clone)]
pub struct AccessLogEntry {
    /// Client IP address
    pub client_ip: IpAddr,

    /// Authenticated user (- if none)
    pub user: Option<String>,

    /// Request timestamp
    pub timestamp: chrono::DateTime<Utc>,

    /// HTTP method
    pub method: String,

    /// Request path
    pub path: String,

    /// HTTP version
    pub http_version: String,

    /// Status code
    pub status_code: u16,

    /// Response size in bytes
    pub response_size: u64,

    /// Referer header
    pub referer: Option<String>,

    /// User-Agent header
    pub user_agent: Option<String>,
}

impl AccessLogEntry {
    /// Format as Apache Common Log Format
    pub fn format_apache_common(&self) -> String {
        format!(
            "{} - {} [{}] \"{} {} {}\" {} {}",
            self.client_ip,
            self.user.as_deref().unwrap_or("-"),
            self.timestamp.format("%d/%b/%Y:%H:%M:%S %z"),
            self.method,
            self.path,
            self.http_version,
            self.status_code,
            if self.response_size > 0 {
                self.response_size.to_string()
            } else {
                "-".to_string()
            }
        )
    }

    /// Format as Apache Combined Log Format
    pub fn format_apache_combined(&self) -> String {
        format!(
            "{} \"{}\" \"{}\"",
            self.format_apache_common(),
            self.referer.as_deref().unwrap_or("-"),
            self.user_agent.as_deref().unwrap_or("-")
        )
    }

    /// Format as JSON
    pub fn format_json(&self) -> String {
        serde_json::json!({
            "client_ip": self.client_ip.to_string(),
            "user": self.user,
            "timestamp": self.timestamp.to_rfc3339(),
            "method": self.method,
            "path": self.path,
            "http_version": self.http_version,
            "status_code": self.status_code,
            "response_size": self.response_size,
            "referer": self.referer,
            "user_agent": self.user_agent,
        })
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_apache_common_format() {
        let entry = AccessLogEntry {
            client_ip: "192.168.1.100".parse::<IpAddr>().unwrap(),
            user: Some("testuser".to_string()),
            timestamp: Utc::now(),
            method: "GET".to_string(),
            path: "/api/v1/health".to_string(),
            http_version: "HTTP/1.1".to_string(),
            status_code: 200,
            response_size: 1024,
            referer: None,
            user_agent: Some("curl/7.68.0".to_string()),
        };

        let log = entry.format_apache_common();
        assert!(log.contains("192.168.1.100"));
        assert!(log.contains("testuser"));
        assert!(log.contains("GET /api/v1/health"));
        assert!(log.contains("200"));
        assert!(log.contains("1024"));
    }
}
