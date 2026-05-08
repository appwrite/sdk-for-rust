//! OAuth2Facebook model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Facebook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Facebook {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Facebook OAuth2 app ID.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Facebook OAuth2 app secret.
    #[serde(rename = "appSecret")]
    pub app_secret: String,
}

impl OAuth2Facebook {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get app_id
    pub fn app_id(&self) -> &String {
        &self.app_id
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
    fn test_o_auth2_facebook_creation() {
        let _model = <OAuth2Facebook as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.app_id();
        let _ = _model.app_secret();
    }

    #[test]
    fn test_o_auth2_facebook_serialization() {
        let model = <OAuth2Facebook as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Facebook, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
