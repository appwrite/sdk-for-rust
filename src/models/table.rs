//! Table model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Table {
    /// Table ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Table creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Table update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Table permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    #[serde(default = "default_permissions")]
    pub permissions: Vec<String>,
    /// Database ID.
    #[serde(rename = "databaseId")]
    #[serde(default = "default_database_id")]
    pub database_id: String,
    /// Table name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Optional description for the table.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Table enabled. Can be 'enabled' or 'disabled'. When disabled, the table is
    /// inaccessible to users, but remains accessible to Server SDKs using API
    /// keys.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Whether row-level permissions are enabled. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "rowSecurity")]
    #[serde(default = "default_row_security")]
    pub row_security: bool,
    /// Table columns.
    #[serde(rename = "columns")]
    #[serde(default = "default_columns")]
    pub columns: Vec<serde_json::Value>,
    /// Table indexes.
    #[serde(rename = "indexes")]
    #[serde(default = "default_indexes")]
    pub indexes: Vec<crate::models::ColumnIndex>,
    /// Maximum row size in bytes. Returns 0 when no limit applies.
    #[serde(rename = "bytesMax")]
    #[serde(default = "default_bytes_max")]
    pub bytes_max: i64,
    /// Currently used row size in bytes based on defined columns.
    #[serde(rename = "bytesUsed")]
    #[serde(default = "default_bytes_used")]
    pub bytes_used: i64,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_permissions() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_database_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_description() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    true
}

fn default_row_security() -> bool {
    false
}

fn default_columns() -> Vec<serde_json::Value> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_indexes() -> Vec<crate::models::ColumnIndex> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_bytes_max() -> i64 {
    0
}

fn default_bytes_used() -> i64 {
    0
}

impl Table {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get permissions
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    /// Get database_id
    pub fn database_id(&self) -> &String {
        &self.database_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get row_security
    pub fn row_security(&self) -> &bool {
        &self.row_security
    }

    /// Get columns
    pub fn columns(&self) -> &Vec<serde_json::Value> {
        &self.columns
    }

    /// Get indexes
    pub fn indexes(&self) -> &Vec<crate::models::ColumnIndex> {
        &self.indexes
    }

    /// Get bytes_max
    pub fn bytes_max(&self) -> &i64 {
        &self.bytes_max
    }

    /// Get bytes_used
    pub fn bytes_used(&self) -> &i64 {
        &self.bytes_used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let _model = <Table as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.database_id();
        let _ = _model.name();
        let _ = _model.description();
        let _ = _model.enabled();
        let _ = _model.row_security();
        let _ = _model.columns();
        let _ = _model.indexes();
        let _ = _model.bytes_max();
        let _ = _model.bytes_used();
    }

    #[test]
    fn test_table_serialization() {
        let model = <Table as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Table, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
