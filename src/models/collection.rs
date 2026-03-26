//! Collection model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Collection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Collection {
    /// Collection ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Collection creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Collection update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Collection permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,
    /// Database ID.
    #[serde(rename = "databaseId")]
    pub database_id: String,
    /// Collection name.
    #[serde(rename = "name")]
    pub name: String,
    /// Collection enabled. Can be 'enabled' or 'disabled'. When disabled, the
    /// collection is inaccessible to users, but remains accessible to Server SDKs
    /// using API keys.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Whether document-level permissions are enabled. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "documentSecurity")]
    pub document_security: bool,
    /// Collection attributes.
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
    /// Collection indexes.
    #[serde(rename = "indexes")]
    pub indexes: Vec<crate::models::Index>,
    /// Maximum document size in bytes. Returns 0 when no limit applies.
    #[serde(rename = "bytesMax")]
    pub bytes_max: i64,
    /// Currently used document size in bytes based on defined attributes.
    #[serde(rename = "bytesUsed")]
    pub bytes_used: i64,
}

impl Collection {
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

    /// Get database_id
    pub fn database_id(&self) -> &String {
        &self.database_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get document_security
    pub fn document_security(&self) -> &bool {
        &self.document_security
    }

    /// Get attributes
    pub fn attributes(&self) -> &Vec<String> {
        &self.attributes
    }

    /// Get indexes
    pub fn indexes(&self) -> &Vec<crate::models::Index> {
        &self.indexes
    }

    /// Get bytes_max
    pub fn bytes_max(&self) -> &i64 {
        &self.bytes_max
    }

    /// Get bytes_used
    pub fn bytes_used(&self) -> &i64 {
        &self.bytes_used
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_creation() {
        let _model = <Collection as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.database_id();
        let _ = _model.name();
        let _ = _model.enabled();
        let _ = _model.document_security();
        let _ = _model.attributes();
        let _ = _model.indexes();
        let _ = _model.bytes_max();
        let _ = _model.bytes_used();
    }

    #[test]
    fn test_collection_serialization() {
        let model = <Collection as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Collection, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
