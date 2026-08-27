//! DedicatedDatabaseBranchList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// BranchList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBranchList {
    /// Total number of branches.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of branches.
    #[serde(rename = "branches")]
    pub branches: Vec<crate::models::DedicatedDatabaseBranch>,
}

impl DedicatedDatabaseBranchList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get branches
    pub fn branches(&self) -> &Vec<crate::models::DedicatedDatabaseBranch> {
        &self.branches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_branch_list_creation() {
        let _model = <DedicatedDatabaseBranchList as Default>::default();
        let _ = _model.total();
        let _ = _model.branches();
    }

    #[test]
    fn test_dedicated_database_branch_list_serialization() {
        let model = <DedicatedDatabaseBranchList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseBranchList, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
