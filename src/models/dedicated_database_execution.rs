//! DedicatedDatabaseExecution model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseExecution {
    /// Result rows as a list of column-name => value maps. Empty for non-returning
    /// statements.
    #[serde(rename = "rows")]
    pub rows: Vec<serde_json::Value>,
    /// Number of rows returned (for SELECT) or affected (for
    /// INSERT/UPDATE/DELETE).
    #[serde(rename = "rowCount")]
    pub row_count: i64,
    /// Column metadata in result-set order.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::models::DedicatedDatabaseExecutionColumn>,
    /// Server-side execution time in milliseconds.
    #[serde(rename = "durationMs")]
    pub duration_ms: i64,
    /// True when the configured row or byte cap was hit and the result was
    /// truncated.
    #[serde(rename = "truncated")]
    pub truncated: bool,
    /// Serialised payload size in bytes.
    #[serde(rename = "bytes")]
    pub bytes: i64,
}

impl DedicatedDatabaseExecution {
    /// Get rows
    pub fn rows(&self) -> &Vec<serde_json::Value> {
        &self.rows
    }

    /// Get row_count
    pub fn row_count(&self) -> &i64 {
        &self.row_count
    }

    /// Get columns
    pub fn columns(&self) -> &Vec<crate::models::DedicatedDatabaseExecutionColumn> {
        &self.columns
    }

    /// Get duration_ms
    pub fn duration_ms(&self) -> &i64 {
        &self.duration_ms
    }

    /// Get truncated
    pub fn truncated(&self) -> &bool {
        &self.truncated
    }

    /// Get bytes
    pub fn bytes(&self) -> &i64 {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_execution_creation() {
        let _model = <DedicatedDatabaseExecution as Default>::default();
        let _ = _model.rows();
        let _ = _model.row_count();
        let _ = _model.columns();
        let _ = _model.duration_ms();
        let _ = _model.truncated();
        let _ = _model.bytes();
    }

    #[test]
    fn test_dedicated_database_execution_serialization() {
        let model = <DedicatedDatabaseExecution as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseExecution, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
