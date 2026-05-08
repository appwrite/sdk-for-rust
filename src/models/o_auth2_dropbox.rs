//! OAuth2Dropbox model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Dropbox
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Dropbox {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Dropbox OAuth2 app key.
    #[serde(rename = "appKey")]
    pub app_key: String,
    /// Dropbox OAuth2 app secret.
    #[serde(rename = "appSecret")]
    pub app_secret: String,
}

impl OAuth2Dropbox {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get app_key
    pub fn app_key(&self) -> &String {
        &self.app_key
    }

    /// Get app_secret
    pub fn app_secret(&self) -> &String {
        &self.app_secret
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_dropbox_creation() {
        let _model = <OAuth2Dropbox as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.app_key();
        let _ = _model.app_secret();
    }

    #[test]
    fn test_o_auth2_dropbox_serialization() {
        let model = <OAuth2Dropbox as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Dropbox, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
