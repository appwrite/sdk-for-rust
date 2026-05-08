//! OAuth2Okta model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Okta
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Okta {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Okta OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Okta OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Okta OAuth2 domain.
    #[serde(rename = "domain")]
    pub domain: String,
    /// Okta OAuth2 authorization server ID.
    #[serde(rename = "authorizationServerId")]
    pub authorization_server_id: String,
}

impl OAuth2Okta {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get client_id
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Get client_secret
    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }

    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get authorization_server_id
    pub fn authorization_server_id(&self) -> &String {
        &self.authorization_server_id
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_okta_creation() {
        let _model = <OAuth2Okta as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.domain();
        let _ = _model.authorization_server_id();
    }

    #[test]
    fn test_o_auth2_okta_serialization() {
        let model = <OAuth2Okta as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Okta, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
