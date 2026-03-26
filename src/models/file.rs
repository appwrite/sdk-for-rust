//! File model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// File
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct File {
    /// File ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Bucket ID.
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    /// File creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// File update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// File permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,
    /// File name.
    #[serde(rename = "name")]
    pub name: String,
    /// File MD5 signature.
    #[serde(rename = "signature")]
    pub signature: String,
    /// File mime type.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// File original size in bytes.
    #[serde(rename = "sizeOriginal")]
    pub size_original: i64,
    /// Total number of chunks available
    #[serde(rename = "chunksTotal")]
    pub chunks_total: i64,
    /// Total number of chunks uploaded
    #[serde(rename = "chunksUploaded")]
    pub chunks_uploaded: i64,
    /// Whether file contents are encrypted at rest.
    #[serde(rename = "encryption")]
    pub encryption: bool,
    /// Compression algorithm used for the file. Will be one of none,
    /// [gzip](https://en.wikipedia.org/wiki/Gzip), or
    /// [zstd](https://en.wikipedia.org/wiki/Zstd).
    #[serde(rename = "compression")]
    pub compression: String,
}

impl File {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get bucket_id
    pub fn bucket_id(&self) -> &String {
        &self.bucket_id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get permissions
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get signature
    pub fn signature(&self) -> &String {
        &self.signature
    }

    /// Get mime_type
    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    /// Get size_original
    pub fn size_original(&self) -> &i64 {
        &self.size_original
    }

    /// Get chunks_total
    pub fn chunks_total(&self) -> &i64 {
        &self.chunks_total
    }

    /// Get chunks_uploaded
    pub fn chunks_uploaded(&self) -> &i64 {
        &self.chunks_uploaded
    }

    /// Get encryption
    pub fn encryption(&self) -> &bool {
        &self.encryption
    }

    /// Get compression
    pub fn compression(&self) -> &String {
        &self.compression
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_creation() {
        let _model = <File as Default>::default();
        let _ = _model.id();
        let _ = _model.bucket_id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.name();
        let _ = _model.signature();
        let _ = _model.mime_type();
        let _ = _model.size_original();
        let _ = _model.chunks_total();
        let _ = _model.chunks_uploaded();
        let _ = _model.encryption();
        let _ = _model.compression();
    }

    #[test]
    fn test_file_serialization() {
        let model = <File as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<File, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
