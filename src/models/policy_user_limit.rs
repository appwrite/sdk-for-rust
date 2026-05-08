//! PolicyUserLimit model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy User Limit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyUserLimit {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Maximum number of users allowed in the project. A value of 0 means the
    /// policy is disabled.
    #[serde(rename = "total")]
    pub total: i64,
}

impl PolicyUserLimit {
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
    fn test_policy_user_limit_creation() {
        let _model = <PolicyUserLimit as Default>::default();
        let _ = _model.id();
        let _ = _model.total();
    }

    #[test]
    fn test_policy_user_limit_serialization() {
        let model = <PolicyUserLimit as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyUserLimit, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
