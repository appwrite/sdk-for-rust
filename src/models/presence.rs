//! Presence model for Appwrite SDK

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Presence {
    /// Presence ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Presence sequence ID.
    #[serde(rename = "$sequence")]
    pub sequence: String,
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
    /// User internal ID.
    #[serde(rename = "userInternalId")]
    pub user_internal_id: String,
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

    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl Presence {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get sequence
    pub fn sequence(&self) -> &String {
        &self.sequence
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

    /// Get user_internal_id
    pub fn user_internal_id(&self) -> &String {
        &self.user_internal_id
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


    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_creation() {
        let _model = <Presence as Default>::default();
        let _ = _model.id();
        let _ = _model.sequence();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.user_internal_id();
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
