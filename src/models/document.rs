//! Document model for Appwrite SDK

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Document {
    /// Document ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Document sequence ID.
    #[serde(rename = "$sequence")]
    #[serde(default = "default_sequence")]
    pub sequence: String,
    /// Collection ID.
    #[serde(rename = "$collectionId")]
    #[serde(default = "default_collection_id")]
    pub collection_id: String,
    /// Database ID.
    #[serde(rename = "$databaseId")]
    #[serde(default = "default_database_id")]
    pub database_id: String,
    /// Document creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Document update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Document permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,

    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_sequence() -> String {
    "".to_string()
}

fn default_collection_id() -> String {
    "".to_string()
}

fn default_database_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

impl Document {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get sequence
    pub fn sequence(&self) -> &String {
        &self.sequence
    }

    /// Get collection_id
    pub fn collection_id(&self) -> &String {
        &self.collection_id
    }

    /// Get database_id
    pub fn database_id(&self) -> &String {
        &self.database_id
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

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
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
    fn test_document_creation() {
        let _model = <Document as Default>::default();
        let _ = _model.id();
        let _ = _model.sequence();
        let _ = _model.collection_id();
        let _ = _model.database_id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
    }

    #[test]
    fn test_document_serialization() {
        let model = <Document as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Document, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
