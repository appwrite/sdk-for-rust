//! Oauth2Introspection model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Introspection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Introspection {
    /// Whether the token is currently active. An inactive token carries no other
    /// claims (RFC 7662).
    #[serde(rename = "active")]
    #[serde(default = "default_active")]
    pub active: bool,
    /// Kind of token that was introspected. Can be one of: `access_token` or
    /// `refresh_token`.
    #[serde(rename = "token_use")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_use: Option<String>,
    /// OAuth2 token type.
    #[serde(rename = "token_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    /// Space-separated scopes granted to the token.
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// OAuth2 client ID the token was issued to.
    #[serde(rename = "client_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ID of the user who authorized the token.
    #[serde(rename = "sub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Audiences the token is intended for. The first entry is the project API
    /// base URL; any further entries are the resource indicators requested at
    /// authorization time (RFC 8707).
    #[serde(rename = "aud")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<Vec<String>>,
    /// Issuer URL of the token.
    #[serde(rename = "iss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Expiration time as a Unix timestamp in seconds.
    #[serde(rename = "exp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// Issued-at time as a Unix timestamp in seconds.
    #[serde(rename = "iat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    /// Unique identifier of the token.
    #[serde(rename = "jti")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    /// Granted RFC 9396 authorization details, restricted to what the user can
    /// currently access.
    #[serde(rename = "authorization_details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_details: Option<Vec<serde_json::Value>>,
}

fn default_active() -> bool {
    false
}

impl Oauth2Introspection {
    /// Get active
    pub fn active(&self) -> &bool {
        &self.active
    }

    /// Set token_use
    pub fn set_token_use(mut self, token_use: String) -> Self {
        self.token_use = Some(token_use);
        self
    }

    /// Get token_use
    pub fn token_use(&self) -> Option<&String> {
        self.token_use.as_ref()
    }

    /// Set token_type
    pub fn set_token_type(mut self, token_type: String) -> Self {
        self.token_type = Some(token_type);
        self
    }

    /// Get token_type
    pub fn token_type(&self) -> Option<&String> {
        self.token_type.as_ref()
    }

    /// Set scope
    pub fn set_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Get scope
    pub fn scope(&self) -> Option<&String> {
        self.scope.as_ref()
    }

    /// Set client_id
    pub fn set_client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Get client_id
    pub fn client_id(&self) -> Option<&String> {
        self.client_id.as_ref()
    }

    /// Set sub
    pub fn set_sub(mut self, sub: String) -> Self {
        self.sub = Some(sub);
        self
    }

    /// Get sub
    pub fn sub(&self) -> Option<&String> {
        self.sub.as_ref()
    }

    /// Set aud
    pub fn set_aud(mut self, aud: Vec<String>) -> Self {
        self.aud = Some(aud);
        self
    }

    /// Get aud
    pub fn aud(&self) -> Option<&Vec<String>> {
        self.aud.as_ref()
    }

    /// Set iss
    pub fn set_iss(mut self, iss: String) -> Self {
        self.iss = Some(iss);
        self
    }

    /// Get iss
    pub fn iss(&self) -> Option<&String> {
        self.iss.as_ref()
    }

    /// Set exp
    pub fn set_exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Get exp
    pub fn exp(&self) -> Option<&i64> {
        self.exp.as_ref()
    }

    /// Set iat
    pub fn set_iat(mut self, iat: i64) -> Self {
        self.iat = Some(iat);
        self
    }

    /// Get iat
    pub fn iat(&self) -> Option<&i64> {
        self.iat.as_ref()
    }

    /// Set jti
    pub fn set_jti(mut self, jti: String) -> Self {
        self.jti = Some(jti);
        self
    }

    /// Get jti
    pub fn jti(&self) -> Option<&String> {
        self.jti.as_ref()
    }

    /// Set authorization_details
    pub fn set_authorization_details(
        mut self,
        authorization_details: Vec<serde_json::Value>,
    ) -> Self {
        self.authorization_details = Some(authorization_details);
        self
    }

    /// Get authorization_details
    pub fn authorization_details(&self) -> Option<&Vec<serde_json::Value>> {
        self.authorization_details.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_introspection_creation() {
        let _model = <Oauth2Introspection as Default>::default();
        let _ = _model.active();
    }

    #[test]
    fn test_oauth2_introspection_serialization() {
        let model = <Oauth2Introspection as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Introspection, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
