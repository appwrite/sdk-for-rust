//! OAuth2Etsy model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Etsy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Etsy {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Etsy OAuth2 keystring.
    #[serde(rename = "keyString")]
    pub key_string: String,
    /// Etsy OAuth2 shared secret.
    #[serde(rename = "sharedSecret")]
    pub shared_secret: String,
}

impl OAuth2Etsy {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get key_string
    pub fn key_string(&self) -> &String {
        &self.key_string
    }

    /// Get shared_secret
    pub fn shared_secret(&self) -> &String {
        &self.shared_secret
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_etsy_creation() {
        let _model = <OAuth2Etsy as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.key_string();
        let _ = _model.shared_secret();
    }

    #[test]
    fn test_o_auth2_etsy_serialization() {
        let model = <OAuth2Etsy as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Etsy, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
