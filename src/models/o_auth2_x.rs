//! OAuth2X model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2X
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2X {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// X OAuth2 customer key.
    #[serde(rename = "customerKey")]
    pub customer_key: String,
    /// X OAuth2 secret key.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

impl OAuth2X {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get customer_key
    pub fn customer_key(&self) -> &String {
        &self.customer_key
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
    fn test_o_auth2_x_creation() {
        let _model = <OAuth2X as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.customer_key();
        let _ = _model.secret_key();
    }

    #[test]
    fn test_o_auth2_x_serialization() {
        let model = <OAuth2X as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2X, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
