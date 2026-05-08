//! PolicySessionAlert model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Session Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicySessionAlert {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether session alert policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicySessionAlert {
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
    fn test_policy_session_alert_creation() {
        let _model = <PolicySessionAlert as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_session_alert_serialization() {
        let model = <PolicySessionAlert as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicySessionAlert, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
