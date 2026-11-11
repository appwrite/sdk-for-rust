//! Environment model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Environment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Environment {
    /// Environment ID, as passed in the X-Appwrite-Environment header.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Environment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Environment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// ID of the project the environment belongs to, as passed in the
    /// X-Appwrite-Project header.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Environment name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
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

fn default_project_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

impl Environment {
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

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let _model = <Environment as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.project_id();
        let _ = _model.name();
    }

    #[test]
    fn test_environment_serialization() {
        let model = <Environment as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Environment, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
