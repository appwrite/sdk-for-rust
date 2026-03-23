//! IndexList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Indexes List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct IndexList {
    /// Total number of indexes that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of indexes.
    #[serde(rename = "indexes")]
    pub indexes: Vec<crate::models::Index>,
}

impl IndexList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get indexes
    pub fn indexes(&self) -> &Vec<crate::models::Index> {
        &self.indexes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_list_creation() {
        let _model = <IndexList as Default>::default();
        let _ = _model.total();
        let _ = _model.indexes();
    }

    #[test]
    fn test_index_list_serialization() {
        let model = <IndexList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<IndexList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
