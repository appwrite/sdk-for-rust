//! LocaleCodeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Locale codes list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct LocaleCodeList {
    /// Total number of localeCodes that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of localeCodes.
    #[serde(rename = "localeCodes")]
    pub locale_codes: Vec<crate::models::LocaleCode>,
}

impl LocaleCodeList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get locale_codes
    pub fn locale_codes(&self) -> &Vec<crate::models::LocaleCode> {
        &self.locale_codes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_code_list_creation() {
        let _model = <LocaleCodeList as Default>::default();
        let _ = _model.total();
        let _ = _model.locale_codes();
    }

    #[test]
    fn test_locale_code_list_serialization() {
        let model = <LocaleCodeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<LocaleCodeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
