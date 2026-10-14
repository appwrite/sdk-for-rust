//! OAuth2Zoho model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Zoho
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Zoho {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Zoho OAuth2 client ID.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Zoho OAuth2 client secret.
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Zoho OAuth2 prompt values.
    #[serde(rename = "prompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Vec<crate::enums::OAuth2ZohoPrompt>>,
}

impl OAuth2Zoho {
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

    /// Get client_secret
    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }

    /// Set prompt
    pub fn set_prompt(mut self, prompt: Vec<crate::enums::OAuth2ZohoPrompt>) -> Self {
        self.prompt = Some(prompt);
        self
    }

    /// Get prompt
    pub fn prompt(&self) -> Option<&Vec<crate::enums::OAuth2ZohoPrompt>> {
        self.prompt.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_zoho_creation() {
        let _model = <OAuth2Zoho as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.client_id();
        let _ = _model.client_secret();
    }

    #[test]
    fn test_o_auth2_zoho_serialization() {
        let model = <OAuth2Zoho as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Zoho, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
