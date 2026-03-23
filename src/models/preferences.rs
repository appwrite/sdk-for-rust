//! Preferences model for Appwrite SDK

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Preferences {

    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl Preferences {

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_creation() {
        let _model = <Preferences as Default>::default();
    }

    #[test]
    fn test_preferences_serialization() {
        let model = <Preferences as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Preferences, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
