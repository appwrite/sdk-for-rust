//! Variable model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Variable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Variable {
    /// Variable ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Variable key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
    /// Optional description for the variable.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Variable value.
    #[serde(rename = "value")]
    #[serde(default = "default_value")]
    pub value: String,
    /// Variable secret flag. Secret variables can only be updated or deleted, but
    /// never read.
    #[serde(rename = "secret")]
    #[serde(default = "default_secret")]
    pub secret: bool,
    /// Service to which the variable belongs. Possible values are "project",
    /// "function"
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    /// ID of resource to which the variable belongs. If resourceType is "project",
    /// it is empty. If resourceType is "function", it is ID of the function.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_key() -> String {
    "".to_string()
}

fn default_description() -> String {
    "".to_string()
}

fn default_value() -> String {
    "".to_string()
}

fn default_secret() -> bool {
    false
}

fn default_resource_type() -> String {
    "".to_string()
}

fn default_resource_id() -> String {
    "".to_string()
}

impl Variable {
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

    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get value
    pub fn value(&self) -> &String {
        &self.value
    }

    /// Get secret
    pub fn secret(&self) -> &bool {
        &self.secret
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_creation() {
        let _model = <Variable as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.key();
        let _ = _model.description();
        let _ = _model.value();
        let _ = _model.secret();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
    }

    #[test]
    fn test_variable_serialization() {
        let model = <Variable as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Variable, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
