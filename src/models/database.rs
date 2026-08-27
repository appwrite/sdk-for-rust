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
    /// Dedicated database lifecycle status. Null when the database has no valid
    /// dedicated backing.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::enums::DatabaseStatus>,
    /// Underlying engine of the dedicated backing: postgresql, mysql, or mongodb.
    /// A managed product (tablesdb, documentsdb, vectorsdb) reports the engine it
    /// runs on, so its type and engine can differ. Null when the database has no
    /// dedicated backing.
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Compute specification identifier of the dedicated backing, e.g.
    /// s-2vcpu-2gb. Null when the database has no dedicated backing.
    #[serde(rename = "specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    /// Number of secondary high availability replicas, excluding the primary. Null
    /// when backing configuration is unavailable.
    #[serde(rename = "replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// Error message when the dedicated backing failed. Null when the database has
    /// no dedicated backing or has not failed.
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Container status of the dedicated backing: active or inactive. Null when
    /// the database has no dedicated backing or the runtime has not reported one.
    #[serde(rename = "containerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_status: Option<String>,
    /// Idle-lifecycle state of the dedicated backing: active, warm, cold, or
    /// hibernated. Null when the database has no dedicated backing or the runtime
    /// has not reported one.
    #[serde(rename = "lifecycleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<String>,
    /// Database backup policies.
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::models::BackupPolicy>>,
    /// Database backup archives.
    #[serde(rename = "archives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archives: Option<Vec<crate::models::BackupArchive>>,
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

    /// Set engine
    pub fn set_engine(mut self, engine: String) -> Self {
        self.engine = Some(engine);
        self
    }

    /// Get engine
    pub fn engine(&self) -> Option<&String> {
        self.engine.as_ref()
    }

    /// Set specification
    pub fn set_specification(mut self, specification: String) -> Self {
        self.specification = Some(specification);
        self
    }

    /// Get specification
    pub fn specification(&self) -> Option<&String> {
        self.specification.as_ref()
    }

    /// Set replicas
    pub fn set_replicas(mut self, replicas: i64) -> Self {
        self.replicas = Some(replicas);
        self
    }

    /// Get replicas
    pub fn replicas(&self) -> Option<&i64> {
        self.replicas.as_ref()
    }

    /// Set error
    pub fn set_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }

    /// Get error
    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    /// Set container_status
    pub fn set_container_status(mut self, container_status: String) -> Self {
        self.container_status = Some(container_status);
        self
    }

    /// Get container_status
    pub fn container_status(&self) -> Option<&String> {
        self.container_status.as_ref()
    }

    /// Set lifecycle_state
    pub fn set_lifecycle_state(mut self, lifecycle_state: String) -> Self {
        self.lifecycle_state = Some(lifecycle_state);
        self
    }

    /// Get lifecycle_state
    pub fn lifecycle_state(&self) -> Option<&String> {
        self.lifecycle_state.as_ref()
    }

    /// Set policies
    pub fn set_policies(mut self, policies: Vec<crate::models::BackupPolicy>) -> Self {
        self.policies = Some(policies);
        self
    }

    /// Get policies
    pub fn policies(&self) -> Option<&Vec<crate::models::BackupPolicy>> {
        self.policies.as_ref()
    }

    /// Set archives
    pub fn set_archives(mut self, archives: Vec<crate::models::BackupArchive>) -> Self {
        self.archives = Some(archives);
        self
    }

    /// Get archives
    pub fn archives(&self) -> Option<&Vec<crate::models::BackupArchive>> {
        self.archives.as_ref()
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
