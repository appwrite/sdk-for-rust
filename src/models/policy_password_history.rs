//! PolicyPasswordHistory model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password History
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordHistory {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Password history length. A value of 0 means the policy is disabled.
    #[serde(rename = "total")]
    pub total: i64,
}

impl PolicyPasswordHistory {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_password_history_creation() {
        let _model = <PolicyPasswordHistory as Default>::default();
        let _ = _model.id();
        let _ = _model.total();
    }

    #[test]
    fn test_policy_password_history_serialization() {
        let model = <PolicyPasswordHistory as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordHistory, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
