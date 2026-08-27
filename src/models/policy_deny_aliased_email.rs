//! PolicyDenyAliasedEmail model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Deny Aliased Email
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyDenyAliasedEmail {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether the deny aliased email policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PolicyDenyAliasedEmail {
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
    fn test_policy_deny_aliased_email_creation() {
        let _model = <PolicyDenyAliasedEmail as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_deny_aliased_email_serialization() {
        let model = <PolicyDenyAliasedEmail as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyDenyAliasedEmail, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
