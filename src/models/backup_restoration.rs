//! BackupRestoration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Restoration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupRestoration {
    /// Restoration ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Restoration creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Restoration update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Backup archive ID.
    #[serde(rename = "archiveId")]
    pub archive_id: String,
    /// Backup policy ID.
    #[serde(rename = "policyId")]
    pub policy_id: String,
    /// The status of the restoration. Possible values: pending, downloading,
    /// processing, completed, failed.
    #[serde(rename = "status")]
    pub status: String,
    /// The backup start time.
    #[serde(rename = "startedAt")]
    pub started_at: String,
    /// Migration ID.
    #[serde(rename = "migrationId")]
    pub migration_id: String,
    /// The services that are backed up by this policy.
    #[serde(rename = "services")]
    pub services: Vec<String>,
    /// The resources that are backed up by this policy.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// Optional data in key-value object.
    #[serde(rename = "options")]
    pub options: String,
}

impl BackupRestoration {
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

    /// Get archive_id
    pub fn archive_id(&self) -> &String {
        &self.archive_id
    }

    /// Get policy_id
    pub fn policy_id(&self) -> &String {
        &self.policy_id
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get started_at
    pub fn started_at(&self) -> &String {
        &self.started_at
    }

    /// Get migration_id
    pub fn migration_id(&self) -> &String {
        &self.migration_id
    }

    /// Get services
    pub fn services(&self) -> &Vec<String> {
        &self.services
    }

    /// Get resources
    pub fn resources(&self) -> &Vec<String> {
        &self.resources
    }

    /// Get options
    pub fn options(&self) -> &String {
        &self.options
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_restoration_creation() {
        let _model = <BackupRestoration as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.archive_id();
        let _ = _model.policy_id();
        let _ = _model.status();
        let _ = _model.started_at();
        let _ = _model.migration_id();
        let _ = _model.services();
        let _ = _model.resources();
        let _ = _model.options();
    }

    #[test]
    fn test_backup_restoration_serialization() {
        let model = <BackupRestoration as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BackupRestoration, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
