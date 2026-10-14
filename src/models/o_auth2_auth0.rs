//! OAuth2Auth0 model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Auth0
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Auth0 {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Auth0 OAuth2 client ID.
    #[serde(rename = "clientId")]
    #[serde(default = "default_client_id")]
    pub client_id: String,
    /// Auth0 OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    #[serde(default = "default_client_secret")]
    pub client_secret: String,
    /// Auth0 OAuth2 prompt values.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: Vec<crate::enums::OAuth2Auth0Prompt>,
    /// Auth0 OAuth2 endpoint domain.
    #[serde(rename = "endpoint")]
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    false
}

fn default_client_id() -> String {
    "".to_string()
}

fn default_client_secret() -> String {
    "".to_string()
}

fn default_prompt() -> Vec<crate::enums::OAuth2Auth0Prompt> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_endpoint() -> String {
    "".to_string()
}

impl OAuth2Auth0 {
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

    /// Get prompt
    pub fn prompt(&self) -> &Vec<crate::enums::OAuth2Auth0Prompt> {
        &self.prompt
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
    fn test_o_auth2_auth0_creation() {
        let _model = <OAuth2Auth0 as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.prompt();
        let _ = _model.endpoint();
    }

    #[test]
    fn test_o_auth2_auth0_serialization() {
        let model = <OAuth2Auth0 as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Auth0, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
