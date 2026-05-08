//! OAuth2Gitlab model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Gitlab
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Gitlab {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// GitLab OAuth2 application ID.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    /// GitLab OAuth2 secret.
    #[serde(rename = "secret")]
    pub secret: String,
    /// GitLab OAuth2 endpoint URL. Defaults to https://gitlab.com for self-hosted
    /// instances.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

impl OAuth2Gitlab {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get application_id
    pub fn application_id(&self) -> &String {
        &self.application_id
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get endpoint
    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_gitlab_creation() {
        let _model = <OAuth2Gitlab as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.application_id();
        let _ = _model.secret();
        let _ = _model.endpoint();
    }

    #[test]
    fn test_o_auth2_gitlab_serialization() {
        let model = <OAuth2Gitlab as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Gitlab, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
