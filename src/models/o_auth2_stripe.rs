//! OAuth2Stripe model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Stripe
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Stripe {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Stripe OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Stripe OAuth2 API secret key.
    #[serde(rename = "apiSecretKey")]
    pub api_secret_key: String,
}

impl OAuth2Stripe {
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

    /// Get api_secret_key
    pub fn api_secret_key(&self) -> &String {
        &self.api_secret_key
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_stripe_creation() {
        let _model = <OAuth2Stripe as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.api_secret_key();
    }

    #[test]
    fn test_o_auth2_stripe_serialization() {
        let model = <OAuth2Stripe as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Stripe, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
