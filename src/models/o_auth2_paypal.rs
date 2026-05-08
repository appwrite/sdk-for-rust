//! OAuth2Paypal model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Paypal
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Paypal {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// PayPal OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// PayPal OAuth2 secret key.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

impl OAuth2Paypal {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get client_id
    pub fn client_id(&self) -> &String {
        &self.client_id
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
    fn test_o_auth2_paypal_creation() {
        let _model = <OAuth2Paypal as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.secret_key();
    }

    #[test]
    fn test_o_auth2_paypal_serialization() {
        let model = <OAuth2Paypal as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Paypal, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
