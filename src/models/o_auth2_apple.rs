//! OAuth2Apple model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Apple
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Apple {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Apple OAuth2 service ID.
    #[serde(rename = "serviceId")]
    pub service_id: String,
    /// Apple OAuth2 key ID.
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// Apple OAuth2 team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Apple OAuth2 .p8 private key file contents. The secret key wrapped by the
    /// PEM markers is 200 characters long.
    #[serde(rename = "p8File")]
    pub p8_file: String,
}

impl OAuth2Apple {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get service_id
    pub fn service_id(&self) -> &String {
        &self.service_id
    }

    /// Get key_id
    pub fn key_id(&self) -> &String {
        &self.key_id
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get p8_file
    pub fn p8_file(&self) -> &String {
        &self.p8_file
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_apple_creation() {
        let _model = <OAuth2Apple as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.service_id();
        let _ = _model.key_id();
        let _ = _model.team_id();
        let _ = _model.p8_file();
    }

    #[test]
    fn test_o_auth2_apple_serialization() {
        let model = <OAuth2Apple as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Apple, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
