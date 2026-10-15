//! DedicatedDatabaseRestoration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Restoration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseRestoration {
    /// Restoration ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Restoration creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Database ID being restored into.
    #[serde(rename = "databaseId")]
    #[serde(default = "default_database_id")]
    pub database_id: String,
    /// Source database ID when restoring a backup into another database.
    #[serde(rename = "sourceDatabaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    /// Project ID.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Backup ID used for restoration (null for PITR).
    #[serde(rename = "backupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// Restoration type. Possible values: backup (restore from a specific backup
    /// snapshot), pitr (point-in-time recovery to a specific timestamp).
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Where the backup was restored. Possible values: in_place (replayed over the
    /// database that took the backup), clone (recovered onto a second workload
    /// beside the database, which keeps serving until the clone is promoted onto
    /// its endpoint), copy (recovered onto a separate database with its own
    /// endpoint).
    #[serde(rename = "restoreMode")]
    #[serde(default = "default_restore_mode")]
    pub restore_mode: String,
    /// Restoration status. Possible values: pending (queued for processing),
    /// running (currently in progress), completed (successfully finished), failed
    /// (encountered an error).
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
    /// Target time for PITR restoration in ISO 8601 format.
    #[serde(rename = "targetTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_time: Option<String>,
    /// Restoration start time in ISO 8601 format.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Restoration completion time in ISO 8601 format.
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Error message if restoration failed.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_database_id() -> String {
    "".to_string()
}

fn default_project_id() -> String {
    "".to_string()
}

fn default_type() -> String {
    "backup".to_string()
}

fn default_restore_mode() -> String {
    "in_place".to_string()
}

fn default_status() -> String {
    "pending".to_string()
}

fn default_error() -> String {
    "".to_string()
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

    /// Set source_database_id
    pub fn set_source_database_id(mut self, source_database_id: String) -> Self {
        self.source_database_id = Some(source_database_id);
        self
    }

    /// Get source_database_id
    pub fn source_database_id(&self) -> Option<&String> {
        self.source_database_id.as_ref()
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Set backup_id
    pub fn set_backup_id(mut self, backup_id: String) -> Self {
        self.backup_id = Some(backup_id);
        self
    }

    /// Get backup_id
    pub fn backup_id(&self) -> Option<&String> {
        self.backup_id.as_ref()
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get restore_mode
    pub fn restore_mode(&self) -> &String {
        &self.restore_mode
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Set target_time
    pub fn set_target_time(mut self, target_time: String) -> Self {
        self.target_time = Some(target_time);
        self
    }

    /// Get target_time
    pub fn target_time(&self) -> Option<&String> {
        self.target_time.as_ref()
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

    /// Set completed_at
    pub fn set_completed_at(mut self, completed_at: String) -> Self {
        self.completed_at = Some(completed_at);
        self
    }

    /// Get completed_at
    pub fn completed_at(&self) -> Option<&String> {
        self.completed_at.as_ref()
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
        let _ = _model.project_id();
        let _ = _model.r#type();
        let _ = _model.restore_mode();
        let _ = _model.status();
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
