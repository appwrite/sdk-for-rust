//! OAuth2Yandex model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Yandex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Yandex {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Yandex OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Yandex OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}

impl OAuth2Yandex {
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_yandex_creation() {
        let _model = <OAuth2Yandex as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
    }

    #[test]
    fn test_o_auth2_yandex_serialization() {
        let model = <OAuth2Yandex as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Yandex, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
