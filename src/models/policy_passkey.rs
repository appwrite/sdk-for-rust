//! PolicyPasskey model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasskey {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Relying party ID passkeys are bound to. Empty until configured.
    #[serde(rename = "rpId")]
    #[serde(default = "default_rp_id")]
    pub rp_id: String,
    /// Web origins passkeys work on, from the project's platforms on the relying
    /// party ID and a localhost web platform. Read-only.
    #[serde(rename = "origins")]
    #[serde(default = "default_origins")]
    pub origins: Vec<String>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_rp_id() -> String {
    "".to_string()
}

fn default_origins() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl PolicyPasskey {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get rp_id
    pub fn rp_id(&self) -> &String {
        &self.rp_id
    }

    /// Get origins
    pub fn origins(&self) -> &Vec<String> {
        &self.origins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_passkey_creation() {
        let _model = <PolicyPasskey as Default>::default();
        let _ = _model.id();
        let _ = _model.rp_id();
        let _ = _model.origins();
    }

    #[test]
    fn test_policy_passkey_serialization() {
        let model = <PolicyPasskey as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasskey, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
