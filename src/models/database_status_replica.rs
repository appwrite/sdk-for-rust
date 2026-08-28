//! DatabaseStatusReplica model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Replica
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatusReplica {
    /// Member index within the database. Read `role` for which member accepts
    /// writes: a failover moves the primary without renumbering the indexes.
    #[serde(rename = "index")]
    pub index: i64,
    /// Member role. Possible values: primary (accepts reads and writes), replica
    /// (read-only follower), unknown (placement not established; reported while a
    /// transition is moving or restarting the topology, so no member can be named
    /// the write target).
    #[serde(rename = "role")]
    pub role: String,
    /// Whether the replica is healthy.
    #[serde(rename = "healthy")]
    pub healthy: bool,
    /// Whether the engine reports this member's replication stream as up. Null
    /// when no reading was taken: a primary has no stream to report, and a member
    /// that is not healthy, or whose probe did not answer, has none yet. `healthy`
    /// is a reachability probe of the member itself and says nothing about
    /// replication, so a healthy member may still not be replicating.
    #[serde(rename = "replicating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicating: Option<bool>,
    /// Replication lag in seconds (null for primary). Also null against
    /// `replicating: true`, for a member that is streaming but whose engine
    /// printed no numeric lag.
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

    /// Set replicating
    pub fn set_replicating(mut self, replicating: bool) -> Self {
        self.replicating = Some(replicating);
        self
    }

    /// Get replicating
    pub fn replicating(&self) -> Option<&bool> {
        self.replicating.as_ref()
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
