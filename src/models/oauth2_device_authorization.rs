//! Oauth2DeviceAuthorization model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Device Authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2DeviceAuthorization {
    /// Device verification code used by the client to poll the token endpoint.
    #[serde(rename = "device_code")]
    pub device_code: String,
    /// Short code the end user enters on the verification page.
    #[serde(rename = "user_code")]
    pub user_code: String,
    /// URL where the end user enters the user code.
    #[serde(rename = "verification_uri")]
    pub verification_uri: String,
    /// Verification URL with the user code prefilled as a query parameter.
    #[serde(rename = "verification_uri_complete")]
    pub verification_uri_complete: String,
    /// Lifetime of the device code and user code in seconds.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    /// Minimum polling interval for the token endpoint in seconds.
    #[serde(rename = "interval")]
    pub interval: i64,
}

impl Oauth2DeviceAuthorization {
    /// Get device_code
    pub fn device_code(&self) -> &String {
        &self.device_code
    }

    /// Get user_code
    pub fn user_code(&self) -> &String {
        &self.user_code
    }

    /// Get verification_uri
    pub fn verification_uri(&self) -> &String {
        &self.verification_uri
    }

    /// Get verification_uri_complete
    pub fn verification_uri_complete(&self) -> &String {
        &self.verification_uri_complete
    }

    /// Get expires_in
    pub fn expires_in(&self) -> &i64 {
        &self.expires_in
    }

    /// Get interval
    pub fn interval(&self) -> &i64 {
        &self.interval
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_device_authorization_creation() {
        let _model = <Oauth2DeviceAuthorization as Default>::default();
        let _ = _model.device_code();
        let _ = _model.user_code();
        let _ = _model.verification_uri();
        let _ = _model.verification_uri_complete();
        let _ = _model.expires_in();
        let _ = _model.interval();
    }

    #[test]
    fn test_oauth2_device_authorization_serialization() {
        let model = <Oauth2DeviceAuthorization as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2DeviceAuthorization, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
