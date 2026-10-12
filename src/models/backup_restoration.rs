//! BackupRestoration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Restoration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupRestoration {
    /// Restoration ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Restoration creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Restoration update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Backup archive ID.
    #[serde(rename = "archiveId")]
    #[serde(default = "default_archive_id")]
    pub archive_id: String,
    /// Backup policy ID.
    #[serde(rename = "policyId")]
    #[serde(default = "default_policy_id")]
    pub policy_id: String,
    /// The status of the restoration. Possible values: pending, downloading,
    /// processing, completed, failed.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
    /// The restoration start time. Null until the restoration starts.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Migration ID.
    #[serde(rename = "migrationId")]
    #[serde(default = "default_migration_id")]
    pub migration_id: String,
    /// The services that are backed up by this policy.
    #[serde(rename = "services")]
    pub services: Vec<String>,
    /// The resources that are backed up by this policy.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// Restoration options as a key-value object.
    #[serde(rename = "options")]
    #[serde(default = "default_options")]
    pub options: serde_json::Value,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_archive_id() -> String {
    "".to_string()
}

fn default_policy_id() -> String {
    "".to_string()
}

fn default_status() -> String {
    "".to_string()
}

fn default_migration_id() -> String {
    "".to_string()
}

fn default_options() -> serde_json::Value {
    serde_json::json!({})
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

    /// Set started_at
    pub fn set_started_at(mut self, started_at: String) -> Self {
        self.started_at = Some(started_at);
        self
    }

    /// Get started_at
    pub fn started_at(&self) -> Option<&String> {
        self.started_at.as_ref()
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
    pub fn options(&self) -> &serde_json::Value {
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
