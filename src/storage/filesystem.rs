//! Filesystem-based storage backend

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::Utc;
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info};

use super::{ObjectInfo, ObjectMetadata, StorageBackend};

/// Filesystem storage backend
pub struct FilesystemBackend {
    base_dir: PathBuf,
}

impl FilesystemBackend {
    /// Create new filesystem backend
    pub fn new(base_dir: PathBuf) -> Self {
        FilesystemBackend { base_dir }
    }

    /// Get object path
    fn object_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.base_dir.join(bucket).join(key)
    }

    /// Get bucket directory
    fn bucket_dir(&self, bucket: &str) -> PathBuf {
        self.base_dir.join(bucket)
    }
}

#[async_trait::async_trait]
impl StorageBackend for FilesystemBackend {
    async fn put_object(&self, bucket: &str, key: &str, data: Bytes) -> Result<ObjectMetadata> {
        let path = self.object_path(bucket, key);

        // Create bucket directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Create object key directories if needed
        if let Some(key_parent) = path.parent() {
            fs::create_dir_all(key_parent).await?;
        }

        // Write data
        fs::write(&path, &data).await
            .context("Failed to write object")?;

        // Calculate ETag (MD5 hash)
        let etag = format!("{:x}", md5::compute(&data));

        let metadata = ObjectMetadata {
            size: data.len() as u64,
            etag,
            content_type: mime_guess::from_path(&path)
                .first_or_octet_stream()
                .to_string(),
            last_modified: Utc::now(),
        };

        debug!("Stored object: {}/{} ({} bytes)", bucket, key, metadata.size);

        Ok(metadata)
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<Bytes> {
        let path = self.object_path(bucket, key);

        let data = fs::read(&path).await
            .context("Object not found")?;

        Ok(Bytes::from(data))
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        let path = self.object_path(bucket, key);

        fs::remove_file(&path).await
            .context("Failed to delete object")?;

        debug!("Deleted object: {}/{}", bucket, key);

        Ok(())
    }

    async fn object_exists(&self, bucket: &str, key: &str) -> Result<bool> {
        let path = self.object_path(bucket, key);
        Ok(path.exists())
    }

    async fn get_object_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMetadata> {
        let path = self.object_path(bucket, key);

        let file_meta = fs::metadata(&path).await
            .context("Object not found")?;

        let data = fs::read(&path).await?;
        let etag = format!("{:x}", md5::compute(&data));

        let metadata = ObjectMetadata {
            size: file_meta.len(),
            etag,
            content_type: mime_guess::from_path(&path)
                .first_or_octet_stream()
                .to_string(),
            last_modified: file_meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .unwrap_or_else(|| Utc::now())
                })
                .unwrap_or_else(Utc::now),
        };

        Ok(metadata)
    }

    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectInfo>> {
        let bucket_dir = self.bucket_dir(bucket);

        if !bucket_dir.exists() {
            return Ok(Vec::new());
        }

        let mut objects = Vec::new();

        // Walk directory
        let mut entries = fs::read_dir(bucket_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                let key = entry.file_name().to_string_lossy().to_string();

                // Check prefix filter
                if let Some(prefix) = prefix {
                    if !key.starts_with(prefix) {
                        continue;
                    }
                }

                let metadata = fs::metadata(entry.path()).await?;
                let data = fs::read(entry.path()).await?;
                let etag = format!("{:x}", md5::compute(&data));

                objects.push(ObjectInfo {
                    key,
                    size: metadata.len(),
                    etag,
                    last_modified: metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| {
                            chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                                .unwrap_or_else(|| Utc::now())
                        })
                        .unwrap_or_else(Utc::now),
                });
            }
        }

        objects.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(objects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_put_and_get_object() {
        let temp_dir = TempDir::new().unwrap();
        let backend = FilesystemBackend::new(temp_dir.path().to_path_buf());

        let data = Bytes::from("Hello, World!");

        // Put object
        let metadata = backend
            .put_object("test-bucket", "test-key", data.clone())
            .await
            .unwrap();

        assert_eq!(metadata.size, 13);

        // Get object
        let retrieved = backend.get_object("test-bucket", "test-key").await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_delete_object() {
        let temp_dir = TempDir::new().unwrap();
        let backend = FilesystemBackend::new(temp_dir.path().to_path_buf());

        let data = Bytes::from("Test data");
        backend
            .put_object("test-bucket", "test-key", data)
            .await
            .unwrap();

        // Delete object
        backend
            .delete_object("test-bucket", "test-key")
            .await
            .unwrap();

        // Should not exist
        let exists = backend.object_exists("test-bucket", "test-key").await.unwrap();
        assert!(!exists);
    }
}
