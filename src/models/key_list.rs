//! KeyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// API Keys List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct KeyList {
    /// Total number of keys that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of keys.
    #[serde(rename = "keys")]
    pub keys: Vec<crate::models::Key>,
}

impl KeyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get keys
    pub fn keys(&self) -> &Vec<crate::models::Key> {
        &self.keys
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_list_creation() {
        let _model = <KeyList as Default>::default();
        let _ = _model.total();
        let _ = _model.keys();
    }

    #[test]
    fn test_key_list_serialization() {
        let model = <KeyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<KeyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
