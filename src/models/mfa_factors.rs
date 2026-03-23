//! MfaFactors model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFAFactors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaFactors {
    /// Can TOTP be used for MFA challenge for this account.
    #[serde(rename = "totp")]
    pub totp: bool,
    /// Can phone (SMS) be used for MFA challenge for this account.
    #[serde(rename = "phone")]
    pub phone: bool,
    /// Can email be used for MFA challenge for this account.
    #[serde(rename = "email")]
    pub email: bool,
    /// Can recovery code be used for MFA challenge for this account.
    #[serde(rename = "recoveryCode")]
    pub recovery_code: bool,
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
