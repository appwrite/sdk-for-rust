//! MfaFactors model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFAFactors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaFactors {
    /// Can TOTP be used for MFA challenge for this account.
    #[serde(rename = "totp")]
    #[serde(default = "default_totp")]
    pub totp: bool,
    /// Can phone (SMS) be used for MFA challenge for this account.
    #[serde(rename = "phone")]
    #[serde(default = "default_phone")]
    pub phone: bool,
    /// Can email be used for MFA challenge for this account.
    #[serde(rename = "email")]
    #[serde(default = "default_email")]
    pub email: bool,
    /// Can recovery code be used for MFA challenge for this account.
    #[serde(rename = "recoveryCode")]
    #[serde(default = "default_recovery_code")]
    pub recovery_code: bool,
    /// Can custom factor be used for MFA challenge for this account.
    #[serde(rename = "custom")]
    #[serde(default = "default_custom")]
    pub custom: bool,
}

fn default_totp() -> bool {
    false
}

fn default_phone() -> bool {
    false
}

fn default_email() -> bool {
    false
}

fn default_recovery_code() -> bool {
    false
}

fn default_custom() -> bool {
    false
}

impl MfaFactors {
    /// Get totp
    pub fn totp(&self) -> &bool {
        &self.totp
    }

    /// Get phone
    pub fn phone(&self) -> &bool {
        &self.phone
    }

    /// Get email
    pub fn email(&self) -> &bool {
        &self.email
    }

    /// Get recovery_code
    pub fn recovery_code(&self) -> &bool {
        &self.recovery_code
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
    fn test_mfa_factors_creation() {
        let _model = <MfaFactors as Default>::default();
        let _ = _model.totp();
        let _ = _model.phone();
        let _ = _model.email();
        let _ = _model.recovery_code();
        let _ = _model.custom();
    }

    #[test]
    fn test_mfa_factors_serialization() {
        let model = <MfaFactors as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MfaFactors, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
