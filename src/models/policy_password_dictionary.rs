//! PolicyPasswordDictionary model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password Dictionary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordDictionary {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether password dictionary policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicyPasswordDictionary {
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
    fn test_policy_password_dictionary_creation() {
        let _model = <PolicyPasswordDictionary as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_password_dictionary_serialization() {
        let model = <PolicyPasswordDictionary as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordDictionary, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
