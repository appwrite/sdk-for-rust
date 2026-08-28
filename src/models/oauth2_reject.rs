//! Oauth2Reject model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Reject
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Reject {
    /// URL the end user should be redirected to after the grant is rejected,
    /// carrying an `access_denied` error.
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

impl Oauth2Reject {
    /// Get redirect_url
    pub fn redirect_url(&self) -> &String {
        &self.redirect_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_reject_creation() {
        let _model = <Oauth2Reject as Default>::default();
        let _ = _model.redirect_url();
    }

    #[test]
    fn test_oauth2_reject_serialization() {
        let model = <Oauth2Reject as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Reject, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
