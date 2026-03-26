//! DatabaseList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Databases List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseList {
    /// Total number of databases that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of databases.
    #[serde(rename = "databases")]
    pub databases: Vec<crate::models::Database>,
}

impl DatabaseList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get databases
    pub fn databases(&self) -> &Vec<crate::models::Database> {
        &self.databases
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_list_creation() {
        let _model = <DatabaseList as Default>::default();
        let _ = _model.total();
        let _ = _model.databases();
    }

    #[test]
    fn test_database_list_serialization() {
        let model = <DatabaseList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
