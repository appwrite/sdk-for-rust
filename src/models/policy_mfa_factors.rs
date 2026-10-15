//! PolicyMfaFactors model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy MFA Factors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyMfaFactors {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Whether TOTP can be used to complete an MFA challenge.
    #[serde(rename = "totp")]
    #[serde(default = "default_totp")]
    pub totp: bool,
    /// Whether email can be used to complete an MFA challenge.
    #[serde(rename = "email")]
    #[serde(default = "default_email")]
    pub email: bool,
    /// Whether phone (SMS) can be used to complete an MFA challenge.
    #[serde(rename = "phone")]
    #[serde(default = "default_phone")]
    pub phone: bool,
    /// Whether the custom factor can be used to complete an MFA challenge.
    #[serde(rename = "custom")]
    #[serde(default = "default_custom")]
    pub custom: bool,
}

fn default_id() -> String {
    "".to_string()
}

fn default_totp() -> bool {
    true
}

fn default_email() -> bool {
    true
}

fn default_phone() -> bool {
    true
}

fn default_custom() -> bool {
    false
}

impl PolicyMfaFactors {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get totp
    pub fn totp(&self) -> &bool {
        &self.totp
    }

    /// Get email
    pub fn email(&self) -> &bool {
        &self.email
    }

    /// Get phone
    pub fn phone(&self) -> &bool {
        &self.phone
    }

    /// Get custom
    pub fn custom(&self) -> &bool {
        &self.custom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_mfa_factors_creation() {
        let _model = <PolicyMfaFactors as Default>::default();
        let _ = _model.id();
        let _ = _model.totp();
        let _ = _model.email();
        let _ = _model.phone();
        let _ = _model.custom();
    }

    #[test]
    fn test_policy_mfa_factors_serialization() {
        let model = <PolicyMfaFactors as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyMfaFactors, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
