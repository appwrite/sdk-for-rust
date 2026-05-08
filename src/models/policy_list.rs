//! PolicyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policies List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyList {
    /// Total number of policies in the given project.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of policies.
    #[serde(rename = "policies")]
    pub policies: Vec<serde_json::Value>,
}

impl PolicyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get policies
    pub fn policies(&self) -> &Vec<serde_json::Value> {
        &self.policies
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_list_creation() {
        let _model = <PolicyList as Default>::default();
        let _ = _model.total();
        let _ = _model.policies();
    }

    #[test]
    fn test_policy_list_serialization() {
        let model = <PolicyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
