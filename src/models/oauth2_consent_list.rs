//! Oauth2ConsentList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 consents list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2ConsentList {
    /// Total number of consents that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of consents.
    #[serde(rename = "consents")]
    pub consents: Vec<crate::models::Oauth2Consent>,
}

impl Oauth2ConsentList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get consents
    pub fn consents(&self) -> &Vec<crate::models::Oauth2Consent> {
        &self.consents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_consent_list_creation() {
        let _model = <Oauth2ConsentList as Default>::default();
        let _ = _model.total();
        let _ = _model.consents();
    }

    #[test]
    fn test_oauth2_consent_list_serialization() {
        let model = <Oauth2ConsentList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2ConsentList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
