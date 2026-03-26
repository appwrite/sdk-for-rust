//! ResourceTokenList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Resource Tokens List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ResourceTokenList {
    /// Total number of tokens that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of tokens.
    #[serde(rename = "tokens")]
    pub tokens: Vec<crate::models::ResourceToken>,
}

impl ResourceTokenList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get tokens
    pub fn tokens(&self) -> &Vec<crate::models::ResourceToken> {
        &self.tokens
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_token_list_creation() {
        let _model = <ResourceTokenList as Default>::default();
        let _ = _model.total();
        let _ = _model.tokens();
    }

    #[test]
    fn test_resource_token_list_serialization() {
        let model = <ResourceTokenList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ResourceTokenList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
