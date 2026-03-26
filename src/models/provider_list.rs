//! ProviderList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Provider list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProviderList {
    /// Total number of providers that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of providers.
    #[serde(rename = "providers")]
    pub providers: Vec<crate::models::Provider>,
}

impl ProviderList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get providers
    pub fn providers(&self) -> &Vec<crate::models::Provider> {
        &self.providers
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_list_creation() {
        let _model = <ProviderList as Default>::default();
        let _ = _model.total();
        let _ = _model.providers();
    }

    #[test]
    fn test_provider_list_serialization() {
        let model = <ProviderList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProviderList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
