//! OAuth2Tradeshift model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Tradeshift
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Tradeshift {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Tradeshift OAuth2 client ID.
    #[serde(rename = "oauth2ClientId")]
    #[serde(default = "default_oauth2_client_id")]
    pub oauth2_client_id: String,
    /// Tradeshift OAuth2 client secret.
    #[serde(rename = "oauth2ClientSecret")]
    #[serde(default = "default_oauth2_client_secret")]
    pub oauth2_client_secret: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    false
}

fn default_oauth2_client_id() -> String {
    "".to_string()
}

fn default_oauth2_client_secret() -> String {
    "".to_string()
}

impl OAuth2Tradeshift {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get oauth2_client_id
    pub fn oauth2_client_id(&self) -> &String {
        &self.oauth2_client_id
    }

    /// Get oauth2_client_secret
    pub fn oauth2_client_secret(&self) -> &String {
        &self.oauth2_client_secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_tradeshift_creation() {
        let _model = <OAuth2Tradeshift as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.oauth2_client_id();
        let _ = _model.oauth2_client_secret();
    }

    #[test]
    fn test_o_auth2_tradeshift_serialization() {
        let model = <OAuth2Tradeshift as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Tradeshift, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
