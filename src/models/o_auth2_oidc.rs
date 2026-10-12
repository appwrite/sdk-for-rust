//! OAuth2Oidc model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Oidc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Oidc {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// OpenID Connect OAuth2 client ID.
    #[serde(rename = "clientId")]
    #[serde(default = "default_client_id")]
    pub client_id: String,
    /// OpenID Connect OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    #[serde(default = "default_client_secret")]
    pub client_secret: String,
    /// OpenID Connect prompt values controlling the authentication and consent
    /// screens.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: Vec<crate::enums::OAuth2OidcPrompt>,
    /// OpenID Connect well-known configuration URL. When set, authorization,
    /// token, and user info endpoints can be discovered automatically.
    #[serde(rename = "wellKnownURL")]
    #[serde(default = "default_well_known_url")]
    pub well_known_url: String,
    /// OpenID Connect authorization endpoint URL.
    #[serde(rename = "authorizationURL")]
    #[serde(default = "default_authorization_url")]
    pub authorization_url: String,
    /// OpenID Connect token endpoint URL.
    #[serde(rename = "tokenURL")]
    #[serde(default = "default_token_url")]
    pub token_url: String,
    /// OpenID Connect user info endpoint URL.
    #[serde(rename = "userInfoURL")]
    #[serde(default = "default_user_info_url")]
    pub user_info_url: String,
    /// Maximum authentication age in seconds. When set, the user must have
    /// authenticated within this many seconds.
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
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

fn default_prompt() -> Vec<crate::enums::OAuth2OidcPrompt> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_well_known_url() -> String {
    "".to_string()
}

fn default_authorization_url() -> String {
    "".to_string()
}

fn default_token_url() -> String {
    "".to_string()
}

fn default_user_info_url() -> String {
    "".to_string()
}

impl OAuth2Oidc {
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
    pub fn prompt(&self) -> &Vec<crate::enums::OAuth2OidcPrompt> {
        &self.prompt
    }

    /// Get well_known_url
    pub fn well_known_url(&self) -> &String {
        &self.well_known_url
    }

    /// Get authorization_url
    pub fn authorization_url(&self) -> &String {
        &self.authorization_url
    }

    /// Get token_url
    pub fn token_url(&self) -> &String {
        &self.token_url
    }

    /// Get user_info_url
    pub fn user_info_url(&self) -> &String {
        &self.user_info_url
    }

    /// Set max_age
    pub fn set_max_age(mut self, max_age: i64) -> Self {
        self.max_age = Some(max_age);
        self
    }

    /// Get max_age
    pub fn max_age(&self) -> Option<&i64> {
        self.max_age.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_oidc_creation() {
        let _model = <OAuth2Oidc as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
        let _ = _model.prompt();
        let _ = _model.well_known_url();
        let _ = _model.authorization_url();
        let _ = _model.token_url();
        let _ = _model.user_info_url();
    }

    #[test]
    fn test_o_auth2_oidc_serialization() {
        let model = <OAuth2Oidc as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Oidc, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
