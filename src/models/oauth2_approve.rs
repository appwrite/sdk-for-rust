//! Oauth2Approve model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Approve
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Approve {
    /// URL the end user should be redirected to after the grant is approved,
    /// carrying the authorization `code` and/or `id_token` along with the original
    /// `state`.
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

impl Oauth2Approve {
    /// Get redirect_url
    pub fn redirect_url(&self) -> &String {
        &self.redirect_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_approve_creation() {
        let _model = <Oauth2Approve as Default>::default();
        let _ = _model.redirect_url();
    }

    #[test]
    fn test_oauth2_approve_serialization() {
        let model = <Oauth2Approve as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Approve, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
