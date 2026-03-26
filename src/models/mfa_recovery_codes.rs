//! MfaRecoveryCodes model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFA Recovery Codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaRecoveryCodes {
    /// Recovery codes.
    #[serde(rename = "recoveryCodes")]
    pub recovery_codes: Vec<String>,
}

impl MfaRecoveryCodes {
    /// Get recovery_codes
    pub fn recovery_codes(&self) -> &Vec<String> {
        &self.recovery_codes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfa_recovery_codes_creation() {
        let _model = <MfaRecoveryCodes as Default>::default();
        let _ = _model.recovery_codes();
    }

    #[test]
    fn test_mfa_recovery_codes_serialization() {
        let model = <MfaRecoveryCodes as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MfaRecoveryCodes, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
