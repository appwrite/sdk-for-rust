//! Provider model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Provider {
    /// Provider ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Provider creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Provider update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// The name for the provider instance.
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the provider service.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Is provider enabled?
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Type of provider.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Provider credentials.
    #[serde(rename = "credentials")]
    pub credentials: serde_json::Value,
    /// Provider options.
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

impl Provider {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get provider
    pub fn provider(&self) -> &String {
        &self.provider
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get credentials
    pub fn credentials(&self) -> &serde_json::Value {
        &self.credentials
    }

    /// Set options
    pub fn set_options(mut self, options: serde_json::Value) -> Self {
        self.options = Some(options);
        self
    }

    /// Get options
    pub fn options(&self) -> Option<&serde_json::Value> {
        self.options.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let _model = <Provider as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.provider();
        let _ = _model.enabled();
        let _ = _model.r#type();
        let _ = _model.credentials();
    }

    #[test]
    fn test_provider_serialization() {
        let model = <Provider as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Provider, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
