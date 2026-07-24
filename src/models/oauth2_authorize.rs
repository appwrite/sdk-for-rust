//! Oauth2Authorize model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Authorize
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Authorize {
    /// OAuth2 grant ID. Set when the user must give explicit consent; pass it to
    /// the approve or reject endpoint. Empty when a redirect URL is returned
    /// instead.
    #[serde(rename = "grantId")]
    pub grant_id: String,
    /// URL the end user should be redirected to when the flow can complete without
    /// consent. Empty when consent is still required.
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

impl Oauth2Authorize {
    /// Get grant_id
    pub fn grant_id(&self) -> &String {
        &self.grant_id
    }

    /// Get redirect_url
    pub fn redirect_url(&self) -> &String {
        &self.redirect_url
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_authorize_creation() {
        let _model = <Oauth2Authorize as Default>::default();
        let _ = _model.grant_id();
        let _ = _model.redirect_url();
    }

    #[test]
    fn test_oauth2_authorize_serialization() {
        let model = <Oauth2Authorize as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Authorize, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
