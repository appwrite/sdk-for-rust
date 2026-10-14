//! OAuth2Linkedin model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Linkedin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Linkedin {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// LinkedIn OAuth2 client ID.
    #[serde(rename = "clientId")]
    #[serde(default = "default_client_id")]
    pub client_id: String,
    /// LinkedIn OAuth2 primary client secret.
    #[serde(rename = "primaryClientSecret")]
    #[serde(default = "default_primary_client_secret")]
    pub primary_client_secret: String,
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

fn default_primary_client_secret() -> String {
    "".to_string()
}

impl OAuth2Linkedin {
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

    /// Get primary_client_secret
    pub fn primary_client_secret(&self) -> &String {
        &self.primary_client_secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_linkedin_creation() {
        let _model = <OAuth2Linkedin as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.primary_client_secret();
    }

    #[test]
    fn test_o_auth2_linkedin_serialization() {
        let model = <OAuth2Linkedin as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Linkedin, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
