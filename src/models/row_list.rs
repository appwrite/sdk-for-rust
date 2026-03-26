//! RowList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Rows List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct RowList {
    /// Total number of rows that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of rows.
    #[serde(rename = "rows")]
    pub rows: Vec<crate::models::Row>,
}

impl RowList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get rows
    pub fn rows(&self) -> &Vec<crate::models::Row> {
        &self.rows
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_list_creation() {
        let _model = <RowList as Default>::default();
        let _ = _model.total();
        let _ = _model.rows();
    }

    #[test]
    fn test_row_list_serialization() {
        let model = <RowList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<RowList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
