//! DedicatedDatabaseRestoration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Restoration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseRestoration {
    /// Restoration ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Restoration creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Database ID being restored into.
    #[serde(rename = "databaseId")]
    pub database_id: String,
    /// Source database ID when restoring a backup into another database.
    #[serde(rename = "sourceDatabaseId")]
    pub source_database_id: String,
    /// Project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Backup ID used for restoration (null for PITR).
    #[serde(rename = "backupId")]
    pub backup_id: String,
    /// Restoration type. Possible values: backup (restore from a specific backup
    /// snapshot), pitr (point-in-time recovery to a specific timestamp).
    #[serde(rename = "type")]
    pub r#type: String,
    /// Restoration status. Possible values: pending (queued for processing),
    /// running (currently in progress), completed (successfully finished), failed
    /// (encountered an error).
    #[serde(rename = "status")]
    pub status: String,
    /// Target time for PITR restoration in ISO 8601 format.
    #[serde(rename = "targetTime")]
    pub target_time: String,
    /// Restoration start time in ISO 8601 format.
    #[serde(rename = "startedAt")]
    pub started_at: String,
    /// Restoration completion time in ISO 8601 format.
    #[serde(rename = "completedAt")]
    pub completed_at: String,
    /// Error message if restoration failed.
    #[serde(rename = "error")]
    pub error: String,
}

impl DedicatedDatabaseRestoration {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get database_id
    pub fn database_id(&self) -> &String {
        &self.database_id
    }

    /// Get source_database_id
    pub fn source_database_id(&self) -> &String {
        &self.source_database_id
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get backup_id
    pub fn backup_id(&self) -> &String {
        &self.backup_id
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get target_time
    pub fn target_time(&self) -> &String {
        &self.target_time
    }

    /// Get started_at
    pub fn started_at(&self) -> &String {
        &self.started_at
    }

    /// Get completed_at
    pub fn completed_at(&self) -> &String {
        &self.completed_at
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_restoration_creation() {
        let _model = <DedicatedDatabaseRestoration as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.database_id();
        let _ = _model.source_database_id();
        let _ = _model.project_id();
        let _ = _model.backup_id();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.target_time();
        let _ = _model.started_at();
        let _ = _model.completed_at();
        let _ = _model.error();
    }

    #[test]
    fn test_dedicated_database_restoration_serialization() {
        let model = <DedicatedDatabaseRestoration as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseRestoration, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
