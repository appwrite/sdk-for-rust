//! PolicyPasswordPersonalData model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password Personal Data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordPersonalData {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether password personal data policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicyPasswordPersonalData {
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
    fn test_policy_password_personal_data_creation() {
        let _model = <PolicyPasswordPersonalData as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_password_personal_data_serialization() {
        let model = <PolicyPasswordPersonalData as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordPersonalData, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
