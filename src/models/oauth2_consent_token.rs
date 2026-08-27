//! Oauth2ConsentToken model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Consent Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2ConsentToken {
    /// Token family ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Token creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Token update date in ISO 8601 format. Refreshing the token family updates
    /// this.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// ID of the consent the token family was issued under.
    #[serde(rename = "consentId")]
    pub consent_id: String,
    /// ID of the user the token family belongs to.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// ID of the registered app the token family was issued to. Empty for URL-form
    /// (CIMD) clients.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Client ID metadata document URL of the client the token family was issued
    /// to. Empty for registered apps.
    #[serde(rename = "cimdUrl")]
    pub cimd_url: String,
    /// OAuth2 scopes granted on the token family.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    /// RFC 8707 resource indicators granted on the token family.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    /// Authorization details granted on the token family, as a JSON string. Each
    /// entry has a `type` plus project-defined fields.
    #[serde(rename = "authorizationDetails")]
    pub authorization_details: String,
    /// Expiration time of the current access token of this family in ISO 8601
    /// format.
    #[serde(rename = "expire")]
    pub expire: String,
}

impl Oauth2ConsentToken {
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

    /// Get consent_id
    pub fn consent_id(&self) -> &String {
        &self.consent_id
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
    fn test_oauth2_consent_token_creation() {
        let _model = <Oauth2ConsentToken as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.consent_id();
        let _ = _model.user_id();
        let _ = _model.app_id();
        let _ = _model.cimd_url();
        let _ = _model.scopes();
        let _ = _model.resources();
        let _ = _model.authorization_details();
        let _ = _model.expire();
    }

    #[test]
    fn test_oauth2_consent_token_serialization() {
        let model = <Oauth2ConsentToken as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2ConsentToken, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
