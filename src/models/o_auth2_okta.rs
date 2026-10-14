//! OAuth2Okta model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Okta
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Okta {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Okta OAuth2 client ID.
    #[serde(rename = "clientId")]
    #[serde(default = "default_client_id")]
    pub client_id: String,
    /// Okta OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    #[serde(default = "default_client_secret")]
    pub client_secret: String,
    /// Okta OAuth2 prompt values.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: Vec<crate::enums::OAuth2OktaPrompt>,
    /// Okta OAuth2 domain.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// Okta OAuth2 authorization server ID.
    #[serde(rename = "authorizationServerId")]
    #[serde(default = "default_authorization_server_id")]
    pub authorization_server_id: String,
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

fn default_prompt() -> Vec<crate::enums::OAuth2OktaPrompt> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_domain() -> String {
    "".to_string()
}

fn default_authorization_server_id() -> String {
    "".to_string()
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

    /// Get prompt
    pub fn prompt(&self) -> &Vec<crate::enums::OAuth2OktaPrompt> {
        &self.prompt
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
        let _ = _model.prompt();
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
