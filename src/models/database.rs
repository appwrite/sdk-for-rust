//! Database model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Database {
    /// Database ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Database name.
    #[serde(rename = "name")]
    pub name: String,
    /// Database creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// If database is enabled. Can be 'enabled' or 'disabled'. When disabled, the
    /// database is inaccessible to users, but remains accessible to Server SDKs
    /// using API keys.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Database type.
    #[serde(rename = "type")]
    pub r#type: crate::enums::DatabaseType,
    /// Database status. Possible values: `provisioning`, `ready` or `failed`
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::enums::DatabaseStatus>,
    /// Database backup policies.
    #[serde(rename = "policies")]
    pub policies: Vec<crate::models::BackupPolicy>,
    /// Database backup archives.
    #[serde(rename = "archives")]
    pub archives: Vec<crate::models::BackupArchive>,
}

impl Database {
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

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get r#type
    pub fn r#type(&self) -> &crate::enums::DatabaseType {
        &self.r#type
    }

    /// Set status
    pub fn set_status(mut self, status: crate::enums::DatabaseStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Get status
    pub fn status(&self) -> Option<&crate::enums::DatabaseStatus> {
        self.status.as_ref()
    }

    /// Get policies
    pub fn policies(&self) -> &Vec<crate::models::BackupPolicy> {
        &self.policies
    }

    /// Get archives
    pub fn archives(&self) -> &Vec<crate::models::BackupArchive> {
        &self.archives
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let _model = <Database as Default>::default();
        let _ = _model.id();
        let _ = _model.name();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.enabled();
        let _ = _model.r#type();
        let _ = _model.policies();
        let _ = _model.archives();
    }

    #[test]
    fn test_database_serialization() {
        let model = <Database as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Database, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
