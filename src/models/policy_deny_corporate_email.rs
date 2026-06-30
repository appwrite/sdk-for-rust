//! PolicyDenyCorporateEmail model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Deny Corporate Email
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyDenyCorporateEmail {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether the deny non-corporate email policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicyDenyCorporateEmail {
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
    fn test_policy_deny_corporate_email_creation() {
        let _model = <PolicyDenyCorporateEmail as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_deny_corporate_email_serialization() {
        let model = <PolicyDenyCorporateEmail as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyDenyCorporateEmail, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
