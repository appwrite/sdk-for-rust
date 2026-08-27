//! DedicatedDatabaseBackupStorage model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// BackupStorageConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBackupStorage {
    /// Storage provider. Possible values: s3 (Amazon S3 or S3-compatible), gcs
    /// (Google Cloud Storage), azure (Azure Blob Storage).
    #[serde(rename = "provider")]
    pub provider: String,
    /// Storage bucket or container name.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// Storage region.
    #[serde(rename = "region")]
    pub region: String,
    /// Object key prefix for backups.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// Custom endpoint for S3-compatible storage.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

impl DedicatedDatabaseBackupStorage {
    /// Get provider
    pub fn provider(&self) -> &String {
        &self.provider
    }

    /// Get bucket
    pub fn bucket(&self) -> &String {
        &self.bucket
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Get prefix
    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    /// Get endpoint
    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_backup_storage_creation() {
        let _model = <DedicatedDatabaseBackupStorage as Default>::default();
        let _ = _model.provider();
        let _ = _model.bucket();
        let _ = _model.region();
        let _ = _model.prefix();
        let _ = _model.endpoint();
    }

    #[test]
    fn test_dedicated_database_backup_storage_serialization() {
        let model = <DedicatedDatabaseBackupStorage as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseBackupStorage, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
