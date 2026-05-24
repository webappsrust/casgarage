//! Object metadata management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extended object metadata (custom headers, tags, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedMetadata {
    /// Custom metadata (x-amz-meta-* headers)
    pub custom: HashMap<String, String>,

    /// Object tags
    pub tags: HashMap<String, String>,

    /// Content encoding
    pub content_encoding: Option<String>,

    /// Content disposition
    pub content_disposition: Option<String>,

    /// Cache control
    pub cache_control: Option<String>,
}

impl ExtendedMetadata {
    /// Create new empty metadata
    pub fn new() -> Self {
        ExtendedMetadata {
            custom: HashMap::new(),
            tags: HashMap::new(),
            content_encoding: None,
            content_disposition: None,
            cache_control: None,
        }
    }

    /// Add custom metadata
    pub fn add_custom(&mut self, key: String, value: String) {
        self.custom.insert(key, value);
    }

    /// Add tag
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }
}

impl Default for ExtendedMetadata {
    fn default() -> Self {
        Self::new()
    }
}
