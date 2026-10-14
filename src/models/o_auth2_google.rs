//! OAuth2Google model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Google
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Google {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Google OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Google OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Google OAuth2 prompt values.
    #[serde(rename = "prompt")]
    pub prompt: Vec<crate::enums::OAuth2GooglePrompt>,
    /// Native Google sign-in is active and can be used to create sessions from an
    /// ID token. Independent of enabled, which only controls the browser-based
    /// flow.
    #[serde(rename = "nativeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_enabled: Option<bool>,
    /// Additional OAuth2 client IDs accepted as ID token audiences for native
    /// sign-in, next to the client ID.
    #[serde(rename = "nativeClientIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_client_ids: Option<Vec<String>>,
}

impl OAuth2Google {
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
    pub fn prompt(&self) -> &Vec<crate::enums::OAuth2GooglePrompt> {
        &self.prompt
    }

    /// Set native_enabled
    pub fn set_native_enabled(mut self, native_enabled: bool) -> Self {
        self.native_enabled = Some(native_enabled);
        self
    }

    /// Get native_enabled
    pub fn native_enabled(&self) -> Option<&bool> {
        self.native_enabled.as_ref()
    }

    /// Set native_client_ids
    pub fn set_native_client_ids(mut self, native_client_ids: Vec<String>) -> Self {
        self.native_client_ids = Some(native_client_ids);
        self
    }

    /// Get native_client_ids
    pub fn native_client_ids(&self) -> Option<&Vec<String>> {
        self.native_client_ids.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_google_creation() {
        let _model = <OAuth2Google as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.prompt();
    }

    #[test]
    fn test_o_auth2_google_serialization() {
        let model = <OAuth2Google as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Google, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
