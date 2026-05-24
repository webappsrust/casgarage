/// CLI argument parsing utilities
use anyhow::Result;

/// Parse duration string (e.g., "1h", "30m", "1d")
pub fn parse_duration(s: &str) -> Result<std::time::Duration> {
    humantime::parse_duration(s).map_err(|e| anyhow::anyhow!("Invalid duration: {}", e))
}

/// Parse size string (e.g., "1GB", "500MB", "1TB")
pub fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();
    let (num_str, unit) = if let Some(pos) = s.find(|c: char| c.is_alphabetic()) {
        s.split_at(pos)
    } else {
        return Ok(s.parse()?);
    };

    let num: f64 = num_str.trim().parse()?;
    let multiplier = match unit.trim() {
        "B" => 1,
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        "TB" => 1024 * 1024 * 1024 * 1024,
        _ => return Err(anyhow::anyhow!("Unknown size unit: {}", unit)),
    };

    Ok((num * multiplier as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("100").unwrap(), 100);
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("1MB").unwrap(), 1024 * 1024);
        assert_eq!(parse_size("1GB").unwrap(), 1024 * 1024 * 1024);
    }
}
