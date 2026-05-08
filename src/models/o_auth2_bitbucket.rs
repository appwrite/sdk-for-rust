//! OAuth2Bitbucket model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Bitbucket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Bitbucket {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Bitbucket OAuth2 key.
    #[serde(rename = "key")]
    pub key: String,
    /// Bitbucket OAuth2 secret.
    #[serde(rename = "secret")]
    pub secret: String,
}

impl OAuth2Bitbucket {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_bitbucket_creation() {
        let _model = <OAuth2Bitbucket as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.key();
        let _ = _model.secret();
    }

    #[test]
    fn test_o_auth2_bitbucket_serialization() {
        let model = <OAuth2Bitbucket as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Bitbucket, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
