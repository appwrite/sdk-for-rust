//! TableList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Tables List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TableList {
    /// Total number of tables that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of tables.
    #[serde(rename = "tables")]
    #[serde(default = "default_tables")]
    pub tables: Vec<crate::models::Table>,
}

fn default_total() -> i64 {
    0
}

fn default_tables() -> Vec<crate::models::Table> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl TableList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get tables
    pub fn tables(&self) -> &Vec<crate::models::Table> {
        &self.tables
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_list_creation() {
        let _model = <TableList as Default>::default();
        let _ = _model.total();
        let _ = _model.tables();
    }

    #[test]
    fn test_table_list_serialization() {
        let model = <TableList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<TableList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
