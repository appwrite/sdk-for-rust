//! Oauth2Grant model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Grant
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Grant {
    /// Grant ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Grant creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Grant update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// ID of the user the grant belongs to.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
    /// ID of the OAuth2 client (app) the grant was requested for.
    #[serde(rename = "appId")]
    #[serde(default = "default_app_id")]
    pub app_id: String,
    /// Requested OAuth2 scopes the user is being asked to consent to.
    #[serde(rename = "scopes")]
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    /// Requested RFC 8707 resource indicators the user is being asked to consent
    /// to.
    #[serde(rename = "resources")]
    #[serde(default = "default_resources")]
    pub resources: Vec<String>,
    /// Requested authorization_details the user is being asked to consent to, as a
    /// JSON string. Each entry has a `type` plus project-defined fields.
    #[serde(rename = "authorizationDetails")]
    #[serde(default = "default_authorization_details")]
    pub authorization_details: String,
    /// OIDC prompt directive the consent screen should honor. Space-separated list
    /// of: login, consent, select_account.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: String,
    /// Redirect URI the user will be sent to after the flow completes.
    #[serde(rename = "redirectUri")]
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,
    /// Unix timestamp of when the user last authenticated.
    #[serde(rename = "authTime")]
    #[serde(default = "default_auth_time")]
    pub auth_time: i64,
    /// Grant expiration time in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_user_id() -> String {
    "".to_string()
}

fn default_app_id() -> String {
    "".to_string()
}

fn default_scopes() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_resources() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_authorization_details() -> String {
    "[]".to_string()
}

fn default_prompt() -> String {
    "".to_string()
}

fn default_redirect_uri() -> String {
    "".to_string()
}

fn default_auth_time() -> i64 {
    0
}

fn default_expire() -> String {
    "".to_string()
}

impl Oauth2Grant {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get app_id
    pub fn app_id(&self) -> &String {
        &self.app_id
    }

    /// Get scopes
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    /// Get resources
    pub fn resources(&self) -> &Vec<String> {
        &self.resources
    }

    /// Get authorization_details
    pub fn authorization_details(&self) -> &String {
        &self.authorization_details
    }

    /// Get prompt
    pub fn prompt(&self) -> &String {
        &self.prompt
    }

    /// Get redirect_uri
    pub fn redirect_uri(&self) -> &String {
        &self.redirect_uri
    }

    /// Get auth_time
    pub fn auth_time(&self) -> &i64 {
        &self.auth_time
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_grant_creation() {
        let _model = <Oauth2Grant as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.user_id();
        let _ = _model.app_id();
        let _ = _model.scopes();
        let _ = _model.resources();
        let _ = _model.authorization_details();
        let _ = _model.prompt();
        let _ = _model.redirect_uri();
        let _ = _model.auth_time();
        let _ = _model.expire();
    }

    #[test]
    fn test_oauth2_grant_serialization() {
        let model = <Oauth2Grant as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Grant, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
