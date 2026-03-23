//! BackupPolicy model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// backup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupPolicy {
    /// Backup policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Backup policy name.
    #[serde(rename = "name")]
    pub name: String,
    /// Policy creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Policy update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// The services that are backed up by this policy.
    #[serde(rename = "services")]
    pub services: Vec<String>,
    /// The resources that are backed up by this policy.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// The resource ID to backup. Set only if this policy should backup a single
    /// resource.
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The resource type to backup. Set only if this policy should backup a single
    /// resource.
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// How many days to keep the backup before it will be automatically deleted.
    #[serde(rename = "retention")]
    pub retention: i64,
    /// Policy backup schedule in CRON format.
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// Is this policy enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl BackupPolicy {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
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

    /// Get retention
    pub fn retention(&self) -> &i64 {
        &self.retention
    }

    /// Get schedule
    pub fn schedule(&self) -> &String {
        &self.schedule
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_policy_creation() {
        let _model = <BackupPolicy as Default>::default();
        let _ = _model.id();
        let _ = _model.name();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.services();
        let _ = _model.resources();
        let _ = _model.retention();
        let _ = _model.schedule();
        let _ = _model.enabled();
    }

    #[test]
    fn test_backup_policy_serialization() {
        let model = <BackupPolicy as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BackupPolicy, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
