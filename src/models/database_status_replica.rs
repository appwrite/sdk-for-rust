//! DatabaseStatusReplica model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Replica
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatusReplica {
    /// StatefulSet pod index (0 = primary, 1+ = replicas).
    #[serde(rename = "index")]
    pub index: i64,
    /// Replica role: primary or replica.
    #[serde(rename = "role")]
    pub role: String,
    /// Whether the replica is healthy.
    #[serde(rename = "healthy")]
    pub healthy: bool,
    /// Replication lag in seconds (null for primary).
    #[serde(rename = "lagSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_seconds: Option<f64>,
}

impl DatabaseStatusReplica {
    /// Get index
    pub fn index(&self) -> &i64 {
        &self.index
    }

    /// Get role
    pub fn role(&self) -> &String {
        &self.role
    }

    /// Get healthy
    pub fn healthy(&self) -> &bool {
        &self.healthy
    }

    /// Set lag_seconds
    pub fn set_lag_seconds(mut self, lag_seconds: f64) -> Self {
        self.lag_seconds = Some(lag_seconds);
        self
    }

    /// Get lag_seconds
    pub fn lag_seconds(&self) -> Option<&f64> {
        self.lag_seconds.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_status_replica_creation() {
        let _model = <DatabaseStatusReplica as Default>::default();
        let _ = _model.index();
        let _ = _model.role();
        let _ = _model.healthy();
    }

    #[test]
    fn test_database_status_replica_serialization() {
        let model = <DatabaseStatusReplica as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseStatusReplica, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
