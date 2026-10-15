//! BackupArchive model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Archive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupArchive {
    /// Archive ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Archive creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Archive update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Archive policy ID.
    #[serde(rename = "policyId")]
    #[serde(default = "default_policy_id")]
    pub policy_id: String,
    /// Archive size in bytes. Null until the size is known.
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The status of the archive creation. Possible values: pending, processing,
    /// uploading, completed, failed, skipped.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
    /// The backup start time. Null until the backup starts.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Migration ID.
    #[serde(rename = "migrationId")]
    #[serde(default = "default_migration_id")]
    pub migration_id: String,
    /// The services that are backed up by this archive.
    #[serde(rename = "services")]
    #[serde(default = "default_services")]
    pub services: Vec<String>,
    /// The resources that are backed up by this archive.
    #[serde(rename = "resources")]
    #[serde(default = "default_resources")]
    pub resources: Vec<String>,
    /// The resource ID to backup. Set only if this archive should backup a single
    /// resource.
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The resource type to backup. Set only if this archive should backup a
    /// single resource.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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

fn default_policy_id() -> String {
    "".to_string()
}

fn default_status() -> String {
    "".to_string()
}

fn default_migration_id() -> String {
    "".to_string()
}

fn default_services() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_resources() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl BackupArchive {
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

    /// Get policy_id
    pub fn policy_id(&self) -> &String {
        &self.policy_id
    }

    /// Set size
    pub fn set_size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    /// Get size
    pub fn size(&self) -> Option<&i64> {
        self.size.as_ref()
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

    /// Set resource_id
    pub fn set_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Get resource_id
    pub fn resource_id(&self) -> Option<&String> {
        self.resource_id.as_ref()
    }

    /// Set resource_type
    pub fn set_resource_type(mut self, resource_type: String) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Get resource_type
    pub fn resource_type(&self) -> Option<&String> {
        self.resource_type.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_archive_creation() {
        let _model = <BackupArchive as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.policy_id();
        let _ = _model.status();
        let _ = _model.migration_id();
        let _ = _model.services();
        let _ = _model.resources();
    }

    #[test]
    fn test_backup_archive_serialization() {
        let model = <BackupArchive as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BackupArchive, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
