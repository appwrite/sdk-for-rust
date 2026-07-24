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
    /// (read-only follower).
    #[serde(rename = "role")]
    pub role: String,
    /// Member pod status. Possible values: provisioning (pod missing or Pending),
    /// starting (Running but not Ready), active (Running and Ready), failed
    /// (Failed phase or CrashLoopBackOff container), or the lowercased pod phase
    /// reported by the cluster.
    #[serde(rename = "status")]
    pub status: String,
    /// Replication lag in seconds.
    #[serde(rename = "lagSeconds")]
    pub lag_seconds: f64,
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

    /// Get lag_seconds
    pub fn lag_seconds(&self) -> &f64 {
        &self.lag_seconds
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
        let _ = _model.lag_seconds();
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
