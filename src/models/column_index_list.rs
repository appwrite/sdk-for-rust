//! ColumnIndexList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Column Indexes List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ColumnIndexList {
    /// Total number of indexes that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of indexes.
    #[serde(rename = "indexes")]
    pub indexes: Vec<crate::models::ColumnIndex>,
}

impl ColumnIndexList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get indexes
    pub fn indexes(&self) -> &Vec<crate::models::ColumnIndex> {
        &self.indexes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_index_list_creation() {
        let _model = <ColumnIndexList as Default>::default();
        let _ = _model.total();
        let _ = _model.indexes();
    }

    #[test]
    fn test_column_index_list_serialization() {
        let model = <ColumnIndexList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ColumnIndexList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
