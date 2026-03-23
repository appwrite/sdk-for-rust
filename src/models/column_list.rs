//! ColumnList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Columns List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ColumnList {
    /// Total number of columns in the given table.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of columns.
    #[serde(rename = "columns")]
    pub columns: Vec<String>,
}

impl ColumnList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get columns
    pub fn columns(&self) -> &Vec<String> {
        &self.columns
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_list_creation() {
        let _model = <ColumnList as Default>::default();
        let _ = _model.total();
        let _ = _model.columns();
    }

    #[test]
    fn test_column_list_serialization() {
        let model = <ColumnList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ColumnList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
