//! Oauth2Consent model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Consent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Consent {
    /// Consent ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Consent creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Consent update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// ID of the user the consent belongs to.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// ID of the registered app the consent was given to. Empty for URL-form
    /// (CIMD) clients.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Client ID metadata document URL of the client the consent was given to.
    /// Empty for registered apps.
    #[serde(rename = "cimdUrl")]
    pub cimd_url: String,
    /// OAuth2 scopes the user consented to.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    /// RFC 8707 resource indicators the user consented to.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// Authorization details the user consented to, as a JSON string. Each entry
    /// has a `type` plus project-defined fields.
    #[serde(rename = "authorizationDetails")]
    pub authorization_details: String,
    /// Consent expiration time in ISO 8601 format. Empty when the consent has no
    /// token-bound expiry yet.
    #[serde(rename = "expire")]
    pub expire: String,
}

impl Oauth2Consent {
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

    /// Get cimd_url
    pub fn cimd_url(&self) -> &String {
        &self.cimd_url
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

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_consent_creation() {
        let _model = <Oauth2Consent as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.user_id();
        let _ = _model.app_id();
        let _ = _model.cimd_url();
        let _ = _model.scopes();
        let _ = _model.resources();
        let _ = _model.authorization_details();
        let _ = _model.expire();
    }

    #[test]
    fn test_oauth2_consent_serialization() {
        let model = <Oauth2Consent as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Consent, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
