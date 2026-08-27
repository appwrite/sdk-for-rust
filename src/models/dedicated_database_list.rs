//! DedicatedDatabaseList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Dedicated databases list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseList {
    /// Total number of databases that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of databases.
    #[serde(rename = "databases")]
    pub databases: Vec<crate::models::DedicatedDatabase>,
}

impl DedicatedDatabaseList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get databases
    pub fn databases(&self) -> &Vec<crate::models::DedicatedDatabase> {
        &self.databases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_list_creation() {
        let _model = <DedicatedDatabaseList as Default>::default();
        let _ = _model.total();
        let _ = _model.databases();
    }

    #[test]
    fn test_dedicated_database_list_serialization() {
        let model = <DedicatedDatabaseList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
