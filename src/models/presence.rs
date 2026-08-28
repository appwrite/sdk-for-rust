//! Presence model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Presence {
    /// Presence ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Presence creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Presence update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Presence permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Presence status.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Presence source.
    #[serde(rename = "source")]
    pub source: String,
    /// Presence expiry date in ISO 8601 format.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Presence metadata.
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Presence {
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

    /// Get permissions
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Set status
    pub fn set_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// Get status
    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Get source
    pub fn source(&self) -> &String {
        &self.source
    }

    /// Set expires_at
    pub fn set_expires_at(mut self, expires_at: String) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Get expires_at
    pub fn expires_at(&self) -> Option<&String> {
        self.expires_at.as_ref()
    }

    /// Set metadata
    pub fn set_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Get metadata
    pub fn metadata(&self) -> Option<&serde_json::Value> {
        self.metadata.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_creation() {
        let _model = <Presence as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.user_id();
        let _ = _model.source();
    }

    #[test]
    fn test_presence_serialization() {
        let model = <Presence as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Presence, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
