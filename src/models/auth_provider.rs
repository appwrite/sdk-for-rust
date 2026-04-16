//! AuthProvider model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AuthProvider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AuthProvider {
    /// Auth Provider.
    #[serde(rename = "key")]
    pub key: String,
    /// Auth Provider name.
    #[serde(rename = "name")]
    pub name: String,
    /// OAuth 2.0 application ID.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// OAuth 2.0 application secret. Might be JSON string if provider requires
    /// extra configuration.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Auth Provider is active and can be used to create session.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl AuthProvider {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get app_id
    pub fn app_id(&self) -> &String {
        &self.app_id
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_provider_creation() {
        let _model = <AuthProvider as Default>::default();
        let _ = _model.key();
        let _ = _model.name();
        let _ = _model.app_id();
        let _ = _model.secret();
        let _ = _model.enabled();
    }

    #[test]
    fn test_auth_provider_serialization() {
        let model = <AuthProvider as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AuthProvider, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
