//! Bucket model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Bucket {
    /// Bucket ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Bucket creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Bucket update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Bucket permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,
    /// Whether file-level security is enabled. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "fileSecurity")]
    pub file_security: bool,
    /// Bucket name.
    #[serde(rename = "name")]
    pub name: String,
    /// Bucket enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Maximum file size supported.
    #[serde(rename = "maximumFileSize")]
    pub maximum_file_size: i64,
    /// Allowed file extensions.
    #[serde(rename = "allowedFileExtensions")]
    pub allowed_file_extensions: Vec<String>,
    /// Compression algorithm chosen for compression. Will be one of none,
    /// [gzip](https://en.wikipedia.org/wiki/Gzip), or
    /// [zstd](https://en.wikipedia.org/wiki/Zstd).
    #[serde(rename = "compression")]
    pub compression: String,
    /// Bucket is encrypted.
    #[serde(rename = "encryption")]
    pub encryption: bool,
    /// Virus scanning is enabled.
    #[serde(rename = "antivirus")]
    pub antivirus: bool,
    /// Image transformations are enabled.
    #[serde(rename = "transformations")]
    pub transformations: bool,
    /// Total size of this bucket in bytes.
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

impl Bucket {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
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

    /// Get file_security
    pub fn file_security(&self) -> &bool {
        &self.file_security
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get maximum_file_size
    pub fn maximum_file_size(&self) -> &i64 {
        &self.maximum_file_size
    }

    /// Get allowed_file_extensions
    pub fn allowed_file_extensions(&self) -> &Vec<String> {
        &self.allowed_file_extensions
    }

    /// Get compression
    pub fn compression(&self) -> &String {
        &self.compression
    }

    /// Get encryption
    pub fn encryption(&self) -> &bool {
        &self.encryption
    }

    /// Get antivirus
    pub fn antivirus(&self) -> &bool {
        &self.antivirus
    }

    /// Get transformations
    pub fn transformations(&self) -> &bool {
        &self.transformations
    }

    /// Get total_size
    pub fn total_size(&self) -> &i64 {
        &self.total_size
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_creation() {
        let _model = <Bucket as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.file_security();
        let _ = _model.name();
        let _ = _model.enabled();
        let _ = _model.maximum_file_size();
        let _ = _model.allowed_file_extensions();
        let _ = _model.compression();
        let _ = _model.encryption();
        let _ = _model.antivirus();
        let _ = _model.transformations();
        let _ = _model.total_size();
    }

    #[test]
    fn test_bucket_serialization() {
        let model = <Bucket as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Bucket, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
