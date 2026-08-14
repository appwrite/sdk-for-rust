//! DatabaseMigrationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Database Migrations List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseMigrationList {
    /// Total number of migrations that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of migrations.
    #[serde(rename = "migrations")]
    pub migrations: Vec<crate::models::DatabaseMigration>,
}

impl DatabaseMigrationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get migrations
    pub fn migrations(&self) -> &Vec<crate::models::DatabaseMigration> {
        &self.migrations
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_migration_list_creation() {
        let _model = <DatabaseMigrationList as Default>::default();
        let _ = _model.total();
        let _ = _model.migrations();
    }

    #[test]
    fn test_database_migration_list_serialization() {
        let model = <DatabaseMigrationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseMigrationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
