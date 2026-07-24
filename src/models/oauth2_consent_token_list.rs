//! Oauth2ConsentTokenList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 consent tokens list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2ConsentTokenList {
    /// Total number of tokens that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of tokens.
    #[serde(rename = "tokens")]
    pub tokens: Vec<crate::models::Oauth2ConsentToken>,
}

impl Oauth2ConsentTokenList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get tokens
    pub fn tokens(&self) -> &Vec<crate::models::Oauth2ConsentToken> {
        &self.tokens
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_consent_token_list_creation() {
        let _model = <Oauth2ConsentTokenList as Default>::default();
        let _ = _model.total();
        let _ = _model.tokens();
    }

    #[test]
    fn test_oauth2_consent_token_list_serialization() {
        let model = <Oauth2ConsentTokenList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2ConsentTokenList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
