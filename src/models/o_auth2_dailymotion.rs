//! OAuth2Dailymotion model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Dailymotion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Dailymotion {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Dailymotion OAuth2 API key.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// Dailymotion OAuth2 API secret.
    #[serde(rename = "apiSecret")]
    pub api_secret: String,
}

impl OAuth2Dailymotion {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get api_key
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    /// Get api_secret
    pub fn api_secret(&self) -> &String {
        &self.api_secret
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_dailymotion_creation() {
        let _model = <OAuth2Dailymotion as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.api_key();
        let _ = _model.api_secret();
    }

    #[test]
    fn test_o_auth2_dailymotion_serialization() {
        let model = <OAuth2Dailymotion as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Dailymotion, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
