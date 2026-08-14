//! DedicatedDatabaseMember model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Member
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseMember {
    /// Member identifier.
    #[serde(rename = "$id")]
    pub id: String,
    /// Member role. Possible values: primary (accepts reads and writes), replica
    /// (read-only follower), unknown (placement not established; reported while a
    /// transition is moving or restarting the topology and this member has not
    /// been probed, so no member can be named the write target).
    #[serde(rename = "role")]
    pub role: String,
    /// Member pod status. Possible values: pending (configured but absent from the
    /// backend topology, so nothing is bringing it up), provisioning (pod missing
    /// or Pending), starting (Running but not Ready), active (Running and Ready),
    /// failed (Failed phase or CrashLoopBackOff container), or the lowercased pod
    /// phase reported by the cluster.
    #[serde(rename = "status")]
    pub status: String,
    /// Replication lag in seconds. Null when the lag is not known: a primary has
    /// none to report, and a member the backend has not probed has none yet.
    #[serde(rename = "lagSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_seconds: Option<f64>,
}

impl DedicatedDatabaseMember {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get role
    pub fn role(&self) -> &String {
        &self.role
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
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
    fn test_dedicated_database_member_creation() {
        let _model = <DedicatedDatabaseMember as Default>::default();
        let _ = _model.id();
        let _ = _model.role();
        let _ = _model.status();
    }

    #[test]
    fn test_dedicated_database_member_serialization() {
        let model = <DedicatedDatabaseMember as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseMember, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
