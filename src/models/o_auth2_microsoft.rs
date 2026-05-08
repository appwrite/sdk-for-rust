//! OAuth2Microsoft model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Microsoft
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Microsoft {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Microsoft OAuth2 application ID.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    /// Microsoft OAuth2 application secret.
    #[serde(rename = "applicationSecret")]
    pub application_secret: String,
    /// Microsoft Entra ID tenant identifier. Use 'common', 'organizations',
    /// 'consumers' or a specific tenant ID.
    #[serde(rename = "tenant")]
    pub tenant: String,
}

impl OAuth2Microsoft {
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

    /// Get application_secret
    pub fn application_secret(&self) -> &String {
        &self.application_secret
    }

    /// Get tenant
    pub fn tenant(&self) -> &String {
        &self.tenant
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_microsoft_creation() {
        let _model = <OAuth2Microsoft as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.application_id();
        let _ = _model.application_secret();
        let _ = _model.tenant();
    }

    #[test]
    fn test_o_auth2_microsoft_serialization() {
        let model = <OAuth2Microsoft as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Microsoft, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
