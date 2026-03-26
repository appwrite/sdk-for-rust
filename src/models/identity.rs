//! Identity model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Identity {
    /// Identity ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Identity creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Identity update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Identity Provider.
    #[serde(rename = "provider")]
    pub provider: String,
    /// ID of the User in the Identity Provider.
    #[serde(rename = "providerUid")]
    pub provider_uid: String,
    /// Email of the User in the Identity Provider.
    #[serde(rename = "providerEmail")]
    pub provider_email: String,
    /// Identity Provider Access Token.
    #[serde(rename = "providerAccessToken")]
    pub provider_access_token: String,
    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
    pub provider_access_token_expiry: String,
    /// Identity Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
    pub provider_refresh_token: String,
}

impl Identity {
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

    /// Get provider
    pub fn provider(&self) -> &String {
        &self.provider
    }

    /// Get provider_uid
    pub fn provider_uid(&self) -> &String {
        &self.provider_uid
    }

    /// Get provider_email
    pub fn provider_email(&self) -> &String {
        &self.provider_email
    }

    /// Get provider_access_token
    pub fn provider_access_token(&self) -> &String {
        &self.provider_access_token
    }

    /// Get provider_access_token_expiry
    pub fn provider_access_token_expiry(&self) -> &String {
        &self.provider_access_token_expiry
    }

    /// Get provider_refresh_token
    pub fn provider_refresh_token(&self) -> &String {
        &self.provider_refresh_token
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_creation() {
        let _model = <Identity as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.user_id();
        let _ = _model.provider();
        let _ = _model.provider_uid();
        let _ = _model.provider_email();
        let _ = _model.provider_access_token();
        let _ = _model.provider_access_token_expiry();
        let _ = _model.provider_refresh_token();
    }

    #[test]
    fn test_identity_serialization() {
        let model = <Identity as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Identity, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
