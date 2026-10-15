//! OAuth2Google model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Google
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Google {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Google OAuth2 client ID.
    #[serde(rename = "clientId")]
    #[serde(default = "default_client_id")]
    pub client_id: String,
    /// Google OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    #[serde(default = "default_client_secret")]
    pub client_secret: String,
    /// Google OAuth2 prompt values.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: Vec<crate::enums::OAuth2GooglePrompt>,
    /// Native Google sign-in is active and can be used to create sessions from an
    /// ID token. Independent of enabled, which only controls the browser-based
    /// flow.
    #[serde(rename = "nativeEnabled")]
    #[serde(default = "default_native_enabled")]
    pub native_enabled: bool,
    /// Additional OAuth2 client IDs accepted as ID token audiences for native
    /// sign-in, next to the client ID.
    #[serde(rename = "nativeClientIds")]
    #[serde(default = "default_native_client_ids")]
    pub native_client_ids: Vec<String>,
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

fn default_prompt() -> Vec<crate::enums::OAuth2GooglePrompt> {
    serde_json::from_value(serde_json::json!(["consent"])).unwrap()
}

fn default_native_enabled() -> bool {
    false
}

fn default_native_client_ids() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
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

    /// Get native_enabled
    pub fn native_enabled(&self) -> &bool {
        &self.native_enabled
    }

    /// Get native_client_ids
    pub fn native_client_ids(&self) -> &Vec<String> {
        &self.native_client_ids
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
        let _ = _model.native_enabled();
        let _ = _model.native_client_ids();
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
