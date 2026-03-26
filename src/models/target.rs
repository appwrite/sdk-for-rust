//! Target model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Target {
    /// Target ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Target creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Target update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Target Name.
    #[serde(rename = "name")]
    pub name: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Provider ID.
    #[serde(rename = "providerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// The target provider type. Can be one of the following: `email`, `sms` or
    /// `push`.
    #[serde(rename = "providerType")]
    pub provider_type: String,
    /// The target identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Is the target expired.
    #[serde(rename = "expired")]
    pub expired: bool,
}

impl Target {
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

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Set provider_id
    pub fn set_provider_id(mut self, provider_id: String) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    /// Get provider_id
    pub fn provider_id(&self) -> Option<&String> {
        self.provider_id.as_ref()
    }

    /// Get provider_type
    pub fn provider_type(&self) -> &String {
        &self.provider_type
    }

    /// Get identifier
    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    /// Get expired
    pub fn expired(&self) -> &bool {
        &self.expired
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_creation() {
        let _model = <Target as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.user_id();
        let _ = _model.provider_type();
        let _ = _model.identifier();
        let _ = _model.expired();
    }

    #[test]
    fn test_target_serialization() {
        let model = <Target as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Target, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
