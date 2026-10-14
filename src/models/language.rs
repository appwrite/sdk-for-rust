//! Language model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Language
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Language {
    /// Language name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Language two-character ISO 639-1 codes.
    #[serde(rename = "code")]
    #[serde(default = "default_code")]
    pub code: String,
    /// Language native name.
    #[serde(rename = "nativeName")]
    #[serde(default = "default_native_name")]
    pub native_name: String,
}

fn default_name() -> String {
    "".to_string()
}

fn default_code() -> String {
    "".to_string()
}

fn default_native_name() -> String {
    "".to_string()
}

impl Language {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Get native_name
    pub fn native_name(&self) -> &String {
        &self.native_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        let _model = <Language as Default>::default();
        let _ = _model.name();
        let _ = _model.code();
        let _ = _model.native_name();
    }

    #[test]
    fn test_language_serialization() {
        let model = <Language as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Language, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
