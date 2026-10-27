//! Migration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Migration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Migration {
    /// Migration ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Migration creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Migration status ( pending, processing, failed, completed )
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
    /// Migration stage ( init, processing, source-check, destination-check,
    /// migrating, finished )
    #[serde(rename = "stage")]
    #[serde(default = "default_stage")]
    pub stage: String,
    /// A string containing the type of source of the migration.
    #[serde(rename = "source")]
    #[serde(default = "default_source")]
    pub source: String,
    /// A string containing the type of destination of the migration.
    #[serde(rename = "destination")]
    #[serde(default = "default_destination")]
    pub destination: String,
    /// Resources to migrate.
    #[serde(rename = "resources")]
    #[serde(default = "default_resources")]
    pub resources: Vec<String>,
    /// ID of the resource being migrated.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Internal ID of the resource being migrated.
    #[serde(rename = "resourceInternalId")]
    #[serde(default = "default_resource_internal_id")]
    pub resource_internal_id: String,
    /// Type of the resource being migrated.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    /// ID of the parent resource that contains the migrated resource.
    #[serde(rename = "parentResourceId")]
    #[serde(default = "default_parent_resource_id")]
    pub parent_resource_id: String,
    /// Internal ID of the parent resource that contains the migrated resource.
    #[serde(rename = "parentResourceInternalId")]
    #[serde(default = "default_parent_resource_internal_id")]
    pub parent_resource_internal_id: String,
    /// Type of the parent resource that contains the migrated resource.
    #[serde(rename = "parentResourceType")]
    #[serde(default = "default_parent_resource_type")]
    pub parent_resource_type: String,
    /// ID of the destination resource created or overwritten by the migration.
    #[serde(rename = "destinationResourceId")]
    #[serde(default = "default_destination_resource_id")]
    pub destination_resource_id: String,
    /// Internal ID of the destination resource created or overwritten by the
    /// migration.
    #[serde(rename = "destinationResourceInternalId")]
    #[serde(default = "default_destination_resource_internal_id")]
    pub destination_resource_internal_id: String,
    /// Type of the destination resource created or overwritten by the migration.
    #[serde(rename = "destinationResourceType")]
    #[serde(default = "default_destination_resource_type")]
    pub destination_resource_type: String,
    /// A group of counters that represent the total progress of the migration.
    #[serde(rename = "statusCounters")]
    #[serde(default = "default_status_counters")]
    pub status_counters: serde_json::Value,
    /// An array of objects containing the report data of the resources that were
    /// migrated.
    #[serde(rename = "resourceData")]
    #[serde(default = "default_resource_data")]
    pub resource_data: Vec<serde_json::Value>,
    /// All errors that occurred during the migration process.
    #[serde(rename = "errors")]
    #[serde(default = "default_errors")]
    pub errors: Vec<String>,
    /// Migration options used during the migration process.
    #[serde(rename = "options")]
    #[serde(default = "default_options")]
    pub options: serde_json::Value,
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

fn default_status() -> String {
    "".to_string()
}

fn default_stage() -> String {
    "".to_string()
}

fn default_source() -> String {
    "".to_string()
}

fn default_destination() -> String {
    "Appwrite".to_string()
}

fn default_resources() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_resource_internal_id() -> String {
    "".to_string()
}

fn default_resource_type() -> String {
    "".to_string()
}

fn default_parent_resource_id() -> String {
    "".to_string()
}

fn default_parent_resource_internal_id() -> String {
    "".to_string()
}

fn default_parent_resource_type() -> String {
    "".to_string()
}

fn default_destination_resource_id() -> String {
    "".to_string()
}

fn default_destination_resource_internal_id() -> String {
    "".to_string()
}

fn default_destination_resource_type() -> String {
    "".to_string()
}

fn default_status_counters() -> serde_json::Value {
    serde_json::json!({})
}

fn default_resource_data() -> Vec<serde_json::Value> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_errors() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_options() -> serde_json::Value {
    serde_json::json!({})
}

impl Migration {
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

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get stage
    pub fn stage(&self) -> &String {
        &self.stage
    }

    /// Get source
    pub fn source(&self) -> &String {
        &self.source
    }

    /// Get destination
    pub fn destination(&self) -> &String {
        &self.destination
    }

    /// Get resources
    pub fn resources(&self) -> &Vec<String> {
        &self.resources
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get resource_internal_id
    pub fn resource_internal_id(&self) -> &String {
        &self.resource_internal_id
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get parent_resource_id
    pub fn parent_resource_id(&self) -> &String {
        &self.parent_resource_id
    }

    /// Get parent_resource_internal_id
    pub fn parent_resource_internal_id(&self) -> &String {
        &self.parent_resource_internal_id
    }

    /// Get parent_resource_type
    pub fn parent_resource_type(&self) -> &String {
        &self.parent_resource_type
    }

    /// Get destination_resource_id
    pub fn destination_resource_id(&self) -> &String {
        &self.destination_resource_id
    }

    /// Get destination_resource_internal_id
    pub fn destination_resource_internal_id(&self) -> &String {
        &self.destination_resource_internal_id
    }

    /// Get destination_resource_type
    pub fn destination_resource_type(&self) -> &String {
        &self.destination_resource_type
    }

    /// Get status_counters
    pub fn status_counters(&self) -> &serde_json::Value {
        &self.status_counters
    }

    /// Get resource_data
    pub fn resource_data(&self) -> &Vec<serde_json::Value> {
        &self.resource_data
    }

    /// Get errors
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    /// Get options
    pub fn options(&self) -> &serde_json::Value {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_creation() {
        let _model = <Migration as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.status();
        let _ = _model.stage();
        let _ = _model.source();
        let _ = _model.destination();
        let _ = _model.resources();
        let _ = _model.resource_id();
        let _ = _model.resource_internal_id();
        let _ = _model.resource_type();
        let _ = _model.parent_resource_id();
        let _ = _model.parent_resource_internal_id();
        let _ = _model.parent_resource_type();
        let _ = _model.destination_resource_id();
        let _ = _model.destination_resource_internal_id();
        let _ = _model.destination_resource_type();
        let _ = _model.status_counters();
        let _ = _model.resource_data();
        let _ = _model.errors();
        let _ = _model.options();
    }

    #[test]
    fn test_migration_serialization() {
        let model = <Migration as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Migration, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
