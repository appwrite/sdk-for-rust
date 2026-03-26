//! MfaType model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFAType
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaType {
    /// Secret token used for TOTP factor.
    #[serde(rename = "secret")]
    pub secret: String,
    /// URI for authenticator apps.
    #[serde(rename = "uri")]
    pub uri: String,
}

impl MfaType {
    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get uri
    pub fn uri(&self) -> &String {
        &self.uri
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfa_type_creation() {
        let _model = <MfaType as Default>::default();
        let _ = _model.secret();
        let _ = _model.uri();
    }

    #[test]
    fn test_mfa_type_serialization() {
        let model = <MfaType as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MfaType, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
