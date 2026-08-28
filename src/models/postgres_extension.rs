//! PostgresExtension model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Postgres extension
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PostgresExtension {
    /// Extension key used with CREATE EXTENSION.
    #[serde(rename = "key")]
    pub key: String,
    /// Human-readable extension name.
    #[serde(rename = "name")]
    pub name: String,
    /// Short description of what the extension provides.
    #[serde(rename = "description")]
    pub description: String,
    /// Category the extension belongs to.
    #[serde(rename = "category")]
    pub category: String,
}

impl PostgresExtension {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get category
    pub fn category(&self) -> &String {
        &self.category
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgres_extension_creation() {
        let _model = <PostgresExtension as Default>::default();
        let _ = _model.key();
        let _ = _model.name();
        let _ = _model.description();
        let _ = _model.category();
    }

    #[test]
    fn test_postgres_extension_serialization() {
        let model = <PostgresExtension as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PostgresExtension, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
