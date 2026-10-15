//! ResourceToken model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ResourceToken
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ResourceToken {
    /// Token ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Resource ID.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Resource type.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
    /// JWT encoded string.
    #[serde(rename = "secret")]
    #[serde(default = "default_secret")]
    pub secret: String,
    /// Most recent access date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt")]
    #[serde(default = "default_accessed_at")]
    pub accessed_at: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_resource_type() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

fn default_secret() -> String {
    "".to_string()
}

fn default_accessed_at() -> String {
    "".to_string()
}

impl ResourceToken {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get accessed_at
    pub fn accessed_at(&self) -> &String {
        &self.accessed_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_token_creation() {
        let _model = <ResourceToken as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.resource_id();
        let _ = _model.resource_type();
        let _ = _model.expire();
        let _ = _model.secret();
        let _ = _model.accessed_at();
    }

    #[test]
    fn test_resource_token_serialization() {
        let model = <ResourceToken as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ResourceToken, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
