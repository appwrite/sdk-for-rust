//! DedicatedDatabaseExecutionColumn model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ExecutionColumn
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseExecutionColumn {
    /// Column name as returned by the database.
    #[serde(rename = "name")]
    pub name: String,
    /// Engine-specific column type (e.g. int4, text, timestamptz).
    #[serde(rename = "type")]
    pub r#type: String,
}

impl DedicatedDatabaseExecutionColumn {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_execution_column_creation() {
        let _model = <DedicatedDatabaseExecutionColumn as Default>::default();
        let _ = _model.name();
        let _ = _model.r#type();
    }

    #[test]
    fn test_dedicated_database_execution_column_serialization() {
        let model = <DedicatedDatabaseExecutionColumn as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseExecutionColumn, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
