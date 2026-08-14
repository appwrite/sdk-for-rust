//! DedicatedDatabaseOperation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseOperation {
    /// Operation ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Operation creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Database ID the operation ran against.
    #[serde(rename = "databaseId")]
    pub database_id: String,
    /// Operation type, such as provision, update, restore, pausing, resuming,
    /// failover, backup-create or cross-region-enable.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Operation status. Possible values: running (in progress), completed
    /// (finished successfully), failed (ended in an error).
    #[serde(rename = "status")]
    pub status: String,
    /// Number of times this operation has been attempted.
    #[serde(rename = "attempts")]
    pub attempts: i64,
    /// Time the operation was requested, in ISO 8601 format.
    #[serde(rename = "requestedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
    /// Time the operation started, in ISO 8601 format.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Time the operation reached a terminal state, in ISO 8601 format.
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Machine-readable failure code. `Interrupted` marks an attempt that ended
    /// before its outcome could be confirmed.
    #[serde(rename = "errorCode")]
    pub error_code: String,
    /// Failure message if the operation failed.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl DedicatedDatabaseOperation {
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

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get attempts
    pub fn attempts(&self) -> &i64 {
        &self.attempts
    }

    /// Set requested_at
    pub fn set_requested_at(mut self, requested_at: String) -> Self {
        self.requested_at = Some(requested_at);
        self
    }

    /// Get requested_at
    pub fn requested_at(&self) -> Option<&String> {
        self.requested_at.as_ref()
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

    /// Get error_code
    pub fn error_code(&self) -> &String {
        &self.error_code
    }

    /// Get error_message
    pub fn error_message(&self) -> &String {
        &self.error_message
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_operation_creation() {
        let _model = <DedicatedDatabaseOperation as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.database_id();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.attempts();
        let _ = _model.error_code();
        let _ = _model.error_message();
    }

    #[test]
    fn test_dedicated_database_operation_serialization() {
        let model = <DedicatedDatabaseOperation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseOperation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
