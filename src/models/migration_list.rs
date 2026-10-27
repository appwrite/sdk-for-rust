//! MigrationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Migrations List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MigrationList {
    /// Total number of migrations that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of migrations.
    #[serde(rename = "migrations")]
    #[serde(default = "default_migrations")]
    pub migrations: Vec<crate::models::Migration>,
}

fn default_total() -> i64 {
    0
}

fn default_migrations() -> Vec<crate::models::Migration> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl MigrationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get migrations
    pub fn migrations(&self) -> &Vec<crate::models::Migration> {
        &self.migrations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_list_creation() {
        let _model = <MigrationList as Default>::default();
        let _ = _model.total();
        let _ = _model.migrations();
    }

    #[test]
    fn test_migration_list_serialization() {
        let model = <MigrationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MigrationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
