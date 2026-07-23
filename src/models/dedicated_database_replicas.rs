//! DedicatedDatabaseReplicas model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Replicas
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseReplicas {
    /// Number of configured replicas. Zero means high availability is disabled.
    #[serde(rename = "replicas")]
    pub replicas: i64,
    /// Replication sync mode. Possible values: async (asynchronous, fastest), sync
    /// (synchronous, strong consistency), quorum (quorum-based, majority of
    /// replicas must confirm).
    #[serde(rename = "syncMode")]
    pub sync_mode: String,
    /// Per-pod statuses for the primary and every replica.
    #[serde(rename = "members")]
    pub members: Vec<crate::models::DedicatedDatabaseMember>,
}

impl DedicatedDatabaseReplicas {
    /// Get replicas
    pub fn replicas(&self) -> &i64 {
        &self.replicas
    }

    /// Get sync_mode
    pub fn sync_mode(&self) -> &String {
        &self.sync_mode
    }

    /// Get members
    pub fn members(&self) -> &Vec<crate::models::DedicatedDatabaseMember> {
        &self.members
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_replicas_creation() {
        let _model = <DedicatedDatabaseReplicas as Default>::default();
        let _ = _model.replicas();
        let _ = _model.sync_mode();
        let _ = _model.members();
    }

    #[test]
    fn test_dedicated_database_replicas_serialization() {
        let model = <DedicatedDatabaseReplicas as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseReplicas, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
