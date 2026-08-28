//! Oauth2PAR model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 PAR
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2PAR {
    /// Authorization request handle to pass to the authorize endpoint.
    #[serde(rename = "request_uri")]
    pub request_uri: String,
    /// Lifetime of the authorization request handle in seconds.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

impl Oauth2PAR {
    /// Get request_uri
    pub fn request_uri(&self) -> &String {
        &self.request_uri
    }

    /// Get expires_in
    pub fn expires_in(&self) -> &i64 {
        &self.expires_in
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_par_creation() {
        let _model = <Oauth2PAR as Default>::default();
        let _ = _model.request_uri();
        let _ = _model.expires_in();
    }

    #[test]
    fn test_oauth2_par_serialization() {
        let model = <Oauth2PAR as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2PAR, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
