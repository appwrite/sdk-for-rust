//! OAuth2ProviderList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Providers List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2ProviderList {
    /// Total number of OAuth2 providers in the given project.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of OAuth2 providers.
    #[serde(rename = "providers")]
    pub providers: Vec<serde_json::Value>,
}

impl OAuth2ProviderList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get providers
    pub fn providers(&self) -> &Vec<serde_json::Value> {
        &self.providers
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_provider_list_creation() {
        let _model = <OAuth2ProviderList as Default>::default();
        let _ = _model.total();
        let _ = _model.providers();
    }

    #[test]
    fn test_o_auth2_provider_list_serialization() {
        let model = <OAuth2ProviderList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2ProviderList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
