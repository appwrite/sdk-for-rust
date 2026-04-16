//! Key model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Key {
    /// Key ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Key creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Key update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Key name.
    #[serde(rename = "name")]
    pub name: String,
    /// Key expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    pub expire: String,
    /// Allowed permission scopes.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    /// Secret key.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Most recent access date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt")]
    pub accessed_at: String,
    /// List of SDK user agents that used this key.
    #[serde(rename = "sdks")]
    pub sdks: Vec<String>,
}

impl Key {
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

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get scopes
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get accessed_at
    pub fn accessed_at(&self) -> &String {
        &self.accessed_at
    }

    /// Get sdks
    pub fn sdks(&self) -> &Vec<String> {
        &self.sdks
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let _model = <Key as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.expire();
        let _ = _model.scopes();
        let _ = _model.secret();
        let _ = _model.accessed_at();
        let _ = _model.sdks();
    }

    #[test]
    fn test_key_serialization() {
        let model = <Key as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Key, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
