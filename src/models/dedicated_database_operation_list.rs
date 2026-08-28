//! DedicatedDatabaseOperationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OperationList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseOperationList {
    /// Total number of operations.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of operations.
    #[serde(rename = "operations")]
    pub operations: Vec<crate::models::DedicatedDatabaseOperation>,
}

impl DedicatedDatabaseOperationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get operations
    pub fn operations(&self) -> &Vec<crate::models::DedicatedDatabaseOperation> {
        &self.operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_operation_list_creation() {
        let _model = <DedicatedDatabaseOperationList as Default>::default();
        let _ = _model.total();
        let _ = _model.operations();
    }

    #[test]
    fn test_dedicated_database_operation_list_serialization() {
        let model = <DedicatedDatabaseOperationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseOperationList, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
