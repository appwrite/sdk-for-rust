//! OAuth2Apple model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Apple
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Apple {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Apple OAuth2 service ID.
    #[serde(rename = "serviceId")]
    #[serde(default = "default_service_id")]
    pub service_id: String,
    /// Apple OAuth2 key ID.
    #[serde(rename = "keyId")]
    #[serde(default = "default_key_id")]
    pub key_id: String,
    /// Apple OAuth2 team ID.
    #[serde(rename = "teamId")]
    #[serde(default = "default_team_id")]
    pub team_id: String,
    /// Apple OAuth2 .p8 private key file contents. The secret key wrapped by the
    /// PEM markers is 200 characters long.
    #[serde(rename = "p8File")]
    #[serde(default = "default_p8_file")]
    pub p8_file: String,
    /// Native Sign in with Apple is active and can be used to create sessions from
    /// an ID token. Independent of enabled, which only controls the browser-based
    /// flow.
    #[serde(rename = "nativeEnabled")]
    #[serde(default = "default_native_enabled")]
    pub native_enabled: bool,
    /// App bundle IDs accepted as ID token audiences for native Sign in with
    /// Apple, next to the Services ID.
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

fn default_service_id() -> String {
    "".to_string()
}

fn default_key_id() -> String {
    "".to_string()
}

fn default_team_id() -> String {
    "".to_string()
}

fn default_p8_file() -> String {
    "".to_string()
}

fn default_native_enabled() -> bool {
    false
}

fn default_native_client_ids() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl OAuth2Apple {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get service_id
    pub fn service_id(&self) -> &String {
        &self.service_id
    }

    /// Get key_id
    pub fn key_id(&self) -> &String {
        &self.key_id
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get p8_file
    pub fn p8_file(&self) -> &String {
        &self.p8_file
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
    fn test_o_auth2_apple_creation() {
        let _model = <OAuth2Apple as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.service_id();
        let _ = _model.key_id();
        let _ = _model.team_id();
        let _ = _model.p8_file();
        let _ = _model.native_enabled();
        let _ = _model.native_client_ids();
    }

    #[test]
    fn test_o_auth2_apple_serialization() {
        let model = <OAuth2Apple as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Apple, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
