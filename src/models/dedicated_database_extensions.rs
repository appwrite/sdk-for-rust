//! DedicatedDatabaseExtensions model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseExtensions {
    /// List of installed extensions.
    #[serde(rename = "installed")]
    pub installed: Vec<String>,
    /// List of available extensions that can be installed.
    #[serde(rename = "available")]
    pub available: Vec<String>,
    /// Curated metadata (display name, description, category) for each available
    /// extension.
    #[serde(rename = "metadata")]
    pub metadata: Vec<crate::models::PostgresExtension>,
}

impl DedicatedDatabaseExtensions {
    /// Get installed
    pub fn installed(&self) -> &Vec<String> {
        &self.installed
    }

    /// Get available
    pub fn available(&self) -> &Vec<String> {
        &self.available
    }

    /// Get metadata
    pub fn metadata(&self) -> &Vec<crate::models::PostgresExtension> {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_extensions_creation() {
        let _model = <DedicatedDatabaseExtensions as Default>::default();
        let _ = _model.installed();
        let _ = _model.available();
        let _ = _model.metadata();
    }

    #[test]
    fn test_dedicated_database_extensions_serialization() {
        let model = <DedicatedDatabaseExtensions as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseExtensions, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
