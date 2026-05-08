//! OAuth2Disqus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Disqus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Disqus {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Disqus OAuth2 public key.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// Disqus OAuth2 secret key.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

impl OAuth2Disqus {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get public_key
    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    /// Get secret_key
    pub fn secret_key(&self) -> &String {
        &self.secret_key
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_disqus_creation() {
        let _model = <OAuth2Disqus as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.public_key();
        let _ = _model.secret_key();
    }

    #[test]
    fn test_o_auth2_disqus_serialization() {
        let model = <OAuth2Disqus as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Disqus, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
