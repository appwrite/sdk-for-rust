//! LanguageList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Languages List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct LanguageList {
    /// Total number of languages that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of languages.
    #[serde(rename = "languages")]
    pub languages: Vec<crate::models::Language>,
}

impl LanguageList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get languages
    pub fn languages(&self) -> &Vec<crate::models::Language> {
        &self.languages
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_list_creation() {
        let _model = <LanguageList as Default>::default();
        let _ = _model.total();
        let _ = _model.languages();
    }

    #[test]
    fn test_language_list_serialization() {
        let model = <LanguageList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<LanguageList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
