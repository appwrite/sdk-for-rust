//! DedicatedDatabaseBackup model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Backup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBackup {
    /// Backup ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Backup creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Database ID this backup belongs to.
    #[serde(rename = "databaseId")]
    pub database_id: String,
    /// Project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Backup policy ID when the backup was created by a schedule.
    #[serde(rename = "policyId")]
    pub policy_id: String,
    /// Backup trigger. Possible values: manual, schedule.
    #[serde(rename = "trigger")]
    pub trigger: String,
    /// Backup type. Possible values: full (complete database snapshot),
    /// incremental (changes since last backup), wal (write-ahead log continuous
    /// archival).
    #[serde(rename = "type")]
    pub r#type: String,
    /// Backup type that was requested. Differs from `type` when the backend could
    /// not run the requested type and took a different one instead, in which case
    /// `fallbackReason` explains why. Empty for backups taken before the requested
    /// type was recorded.
    #[serde(rename = "requestedType")]
    pub requested_type: String,
    /// Why the backend ran a different backup type than the one requested. Empty
    /// when the backup ran as requested.
    #[serde(rename = "fallbackReason")]
    pub fallback_reason: String,
    /// Backup status. Possible values: pending (queued for processing), running
    /// (currently in progress), completed (successfully finished), failed
    /// (encountered an error), verified (integrity check passed).
    #[serde(rename = "status")]
    pub status: String,
    /// Backup size in bytes.
    #[serde(rename = "sizeBytes")]
    pub size_bytes: i64,
    /// Backup start time in ISO 8601 format.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Backup completion time in ISO 8601 format.
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Backup verification time in ISO 8601 format.
    #[serde(rename = "verifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
    /// Backup expiration time in ISO 8601 format.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Transaction-log position the backup anchors at, in the engine's own
    /// notation: PostgreSQL `{walSegment}|{lsn}`, MySQL and MariaDB
    /// `{binlogFile}|{offset}`, MongoDB `{seconds}|{increment}`. Empty when the
    /// backup recorded no position, which is the case for backup types that carry
    /// none.
    #[serde(rename = "logPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_position: Option<String>,
    /// Error message if backup failed.
    #[serde(rename = "error")]
    pub error: String,
}

impl DedicatedDatabaseBackup {
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

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get policy_id
    pub fn policy_id(&self) -> &String {
        &self.policy_id
    }

    /// Get trigger
    pub fn trigger(&self) -> &String {
        &self.trigger
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get requested_type
    pub fn requested_type(&self) -> &String {
        &self.requested_type
    }

    /// Get fallback_reason
    pub fn fallback_reason(&self) -> &String {
        &self.fallback_reason
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get size_bytes
    pub fn size_bytes(&self) -> &i64 {
        &self.size_bytes
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

    /// Set verified_at
    pub fn set_verified_at(mut self, verified_at: String) -> Self {
        self.verified_at = Some(verified_at);
        self
    }

    /// Get verified_at
    pub fn verified_at(&self) -> Option<&String> {
        self.verified_at.as_ref()
    }

    /// Set expires_at
    pub fn set_expires_at(mut self, expires_at: String) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Get expires_at
    pub fn expires_at(&self) -> Option<&String> {
        self.expires_at.as_ref()
    }

    /// Set log_position
    pub fn set_log_position(mut self, log_position: String) -> Self {
        self.log_position = Some(log_position);
        self
    }

    /// Get log_position
    pub fn log_position(&self) -> Option<&String> {
        self.log_position.as_ref()
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
    fn test_dedicated_database_backup_creation() {
        let _model = <DedicatedDatabaseBackup as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.database_id();
        let _ = _model.project_id();
        let _ = _model.policy_id();
        let _ = _model.trigger();
        let _ = _model.r#type();
        let _ = _model.requested_type();
        let _ = _model.fallback_reason();
        let _ = _model.status();
        let _ = _model.size_bytes();
        let _ = _model.error();
    }

    #[test]
    fn test_dedicated_database_backup_serialization() {
        let model = <DedicatedDatabaseBackup as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseBackup, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
