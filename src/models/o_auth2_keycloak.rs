//! OAuth2Keycloak model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Keycloak
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Keycloak {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Keycloak OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Keycloak OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Keycloak OAuth2 endpoint domain.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Keycloak OAuth2 realm name.
    #[serde(rename = "realmName")]
    pub realm_name: String,
}

impl OAuth2Keycloak {
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

    /// Get endpoint
    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    /// Get realm_name
    pub fn realm_name(&self) -> &String {
        &self.realm_name
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_keycloak_creation() {
        let _model = <OAuth2Keycloak as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.endpoint();
        let _ = _model.realm_name();
    }

    #[test]
    fn test_o_auth2_keycloak_serialization() {
        let model = <OAuth2Keycloak as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Keycloak, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
