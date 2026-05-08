//! OAuth2FusionAuth model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2FusionAuth
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2FusionAuth {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// FusionAuth OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// FusionAuth OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// FusionAuth OAuth2 endpoint domain.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

impl OAuth2FusionAuth {
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_fusion_auth_creation() {
        let _model = <OAuth2FusionAuth as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.endpoint();
    }

    #[test]
    fn test_o_auth2_fusion_auth_serialization() {
        let model = <OAuth2FusionAuth as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2FusionAuth, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
