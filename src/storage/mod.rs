//! Storage backend for object data
//!
//! File-based storage backend for S3 objects
//! Future: Will integrate with Garage storage engine

pub mod filesystem;
pub mod metadata;

use anyhow::Result;
use bytes::Bytes;
use std::path::PathBuf;

/// Storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store an object
    async fn put_object(&self, bucket: &str, key: &str, data: Bytes) -> Result<ObjectMetadata>;

    /// Retrieve an object
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Bytes>;

    /// Delete an object
    async fn delete_object(&self, bucket: &str, key: &str) -> Result<()>;

    /// Check if object exists
    async fn object_exists(&self, bucket: &str, key: &str) -> Result<bool>;

    /// Get object metadata
    async fn get_object_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMetadata>;

    /// List objects in bucket
    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectInfo>>;
}

/// Object metadata
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub size: u64,
    pub etag: String,
    pub content_type: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

/// Object information for listing
#[derive(Debug, Clone)]
pub struct ObjectInfo {
    pub key: String,
    pub size: u64,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

/// Create storage backend
pub fn create_storage_backend(blocks_dir: PathBuf) -> Box<dyn StorageBackend> {
    Box::new(filesystem::FilesystemBackend::new(blocks_dir))
}
