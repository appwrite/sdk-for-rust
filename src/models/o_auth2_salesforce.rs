//! OAuth2Salesforce model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2Salesforce
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct OAuth2Salesforce {
    /// OAuth2 provider ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// OAuth2 provider is active and can be used to create sessions.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Salesforce OAuth2 consumer key.
    #[serde(rename = "customerKey")]
    #[serde(default = "default_customer_key")]
    pub customer_key: String,
    /// Salesforce OAuth2 consumer secret.
    #[serde(rename = "customerSecret")]
    #[serde(default = "default_customer_secret")]
    pub customer_secret: String,
    /// Salesforce OAuth2 prompt values.
    #[serde(rename = "prompt")]
    #[serde(default = "default_prompt")]
    pub prompt: Vec<crate::enums::OAuth2SalesforcePrompt>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    false
}

fn default_customer_key() -> String {
    "".to_string()
}

fn default_customer_secret() -> String {
    "".to_string()
}

fn default_prompt() -> Vec<crate::enums::OAuth2SalesforcePrompt> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl OAuth2Salesforce {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get customer_key
    pub fn customer_key(&self) -> &String {
        &self.customer_key
    }

    /// Get customer_secret
    pub fn customer_secret(&self) -> &String {
        &self.customer_secret
    }

    /// Get prompt
    pub fn prompt(&self) -> &Vec<crate::enums::OAuth2SalesforcePrompt> {
        &self.prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_auth2_salesforce_creation() {
        let _model = <OAuth2Salesforce as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.customer_key();
        let _ = _model.customer_secret();
        let _ = _model.prompt();
    }

    #[test]
    fn test_o_auth2_salesforce_serialization() {
        let model = <OAuth2Salesforce as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<OAuth2Salesforce, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
