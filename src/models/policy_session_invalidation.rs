//! PolicySessionInvalidation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Session Invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicySessionInvalidation {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether session invalidation policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicySessionInvalidation {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_session_invalidation_creation() {
        let _model = <PolicySessionInvalidation as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_session_invalidation_serialization() {
        let model = <PolicySessionInvalidation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicySessionInvalidation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
