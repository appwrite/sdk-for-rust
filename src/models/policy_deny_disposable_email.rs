//! PolicyDenyDisposableEmail model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Deny Disposable Email
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyDenyDisposableEmail {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether the deny disposable email policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicyDenyDisposableEmail {
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
    fn test_policy_deny_disposable_email_creation() {
        let _model = <PolicyDenyDisposableEmail as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_deny_disposable_email_serialization() {
        let model = <PolicyDenyDisposableEmail as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyDenyDisposableEmail, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
