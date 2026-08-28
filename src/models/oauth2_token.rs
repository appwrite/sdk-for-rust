//! Oauth2Token model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Token {
    /// OAuth2 access token.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// OAuth2 token type.
    #[serde(rename = "token_type")]
    pub token_type: String,
    /// Access token lifetime in seconds.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    /// OAuth2 refresh token.
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    /// Space-separated scopes granted to the access token.
    #[serde(rename = "scope")]
    pub scope: String,
    /// Granted RFC 9396 authorization details as a JSON string.
    #[serde(rename = "authorization_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_details: Option<String>,
    /// OpenID Connect ID token. Returned when the `openid` scope is granted.
    #[serde(rename = "id_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
}

impl Oauth2Token {
    /// Get access_token
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    /// Get token_type
    pub fn token_type(&self) -> &String {
        &self.token_type
    }

    /// Get expires_in
    pub fn expires_in(&self) -> &i64 {
        &self.expires_in
    }

    /// Get refresh_token
    pub fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    /// Get scope
    pub fn scope(&self) -> &String {
        &self.scope
    }

    /// Set authorization_details
    pub fn set_authorization_details(mut self, authorization_details: String) -> Self {
        self.authorization_details = Some(authorization_details);
        self
    }

    /// Get authorization_details
    pub fn authorization_details(&self) -> Option<&String> {
        self.authorization_details.as_ref()
    }

    /// Set id_token
    pub fn set_id_token(mut self, id_token: String) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Get id_token
    pub fn id_token(&self) -> Option<&String> {
        self.id_token.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_token_creation() {
        let _model = <Oauth2Token as Default>::default();
        let _ = _model.access_token();
        let _ = _model.token_type();
        let _ = _model.expires_in();
        let _ = _model.refresh_token();
        let _ = _model.scope();
    }

    #[test]
    fn test_oauth2_token_serialization() {
        let model = <Oauth2Token as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Token, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
