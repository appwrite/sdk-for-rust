//! PasskeyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Passkeys List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PasskeyList {
    /// Total number of passkeys that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of passkeys.
    #[serde(rename = "passkeys")]
    pub passkeys: Vec<crate::models::Passkey>,
}

impl PasskeyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get passkeys
    pub fn passkeys(&self) -> &Vec<crate::models::Passkey> {
        &self.passkeys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_list_creation() {
        let _model = <PasskeyList as Default>::default();
        let _ = _model.total();
        let _ = _model.passkeys();
    }

    #[test]
    fn test_passkey_list_serialization() {
        let model = <PasskeyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PasskeyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
