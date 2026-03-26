//! LocaleCode model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// LocaleCode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct LocaleCode {
    /// Locale codes in [ISO
    /// 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)
    #[serde(rename = "code")]
    pub code: String,
    /// Locale name
    #[serde(rename = "name")]
    pub name: String,
}

impl LocaleCode {
    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_code_creation() {
        let _model = <LocaleCode as Default>::default();
        let _ = _model.code();
        let _ = _model.name();
    }

    #[test]
    fn test_locale_code_serialization() {
        let model = <LocaleCode as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<LocaleCode, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
