//! DedicatedDatabaseSpecification model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseSpecification {
    /// Specification slug. Use this value when creating a dedicated database.
    #[serde(rename = "slug")]
    #[serde(default = "default_slug")]
    pub slug: String,
    /// Human readable specification name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Monthly price of the specification in USD.
    #[serde(rename = "price")]
    #[serde(default = "default_price")]
    pub price: f64,
    /// Price per GB of storage above the included amount, per month, in USD.
    #[serde(rename = "storageOverageRate")]
    #[serde(default = "default_storage_overage_rate")]
    pub storage_overage_rate: f64,
    /// Price per GB of bandwidth above the included amount, per month, in USD.
    #[serde(rename = "bandwidthOverageRate")]
    #[serde(default = "default_bandwidth_overage_rate")]
    pub bandwidth_overage_rate: f64,
    /// High availability replica price as a fraction of the specification price.
    #[serde(rename = "replicaRate")]
    #[serde(default = "default_replica_rate")]
    pub replica_rate: f64,
    /// Point-in-time recovery price as a fraction of the specification price.
    #[serde(rename = "pitrRate")]
    #[serde(default = "default_pitr_rate")]
    pub pitr_rate: f64,
    /// Allocated CPU in millicores.
    #[serde(rename = "cpu")]
    #[serde(default = "default_cpu")]
    pub cpu: i64,
    /// Allocated memory in MB.
    #[serde(rename = "memory")]
    #[serde(default = "default_memory")]
    pub memory: i64,
    /// Maximum number of concurrent connections.
    #[serde(rename = "maxConnections")]
    #[serde(default = "default_max_connections")]
    pub max_connections: i64,
    /// Included storage in GB before overage charges apply.
    #[serde(rename = "includedStorage")]
    #[serde(default = "default_included_storage")]
    pub included_storage: i64,
    /// Included bandwidth in GB before overage charges apply.
    #[serde(rename = "includedBandwidth")]
    #[serde(default = "default_included_bandwidth")]
    pub included_bandwidth: i64,
    /// Whether the specification is available on the current plan.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_slug() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_price() -> f64 {
    0.0
}

fn default_storage_overage_rate() -> f64 {
    0.0
}

fn default_bandwidth_overage_rate() -> f64 {
    0.0
}

fn default_replica_rate() -> f64 {
    0.0
}

fn default_pitr_rate() -> f64 {
    0.0
}

fn default_cpu() -> i64 {
    0
}

fn default_memory() -> i64 {
    0
}

fn default_max_connections() -> i64 {
    0
}

fn default_included_storage() -> i64 {
    0
}

fn default_included_bandwidth() -> i64 {
    0
}

fn default_enabled() -> bool {
    false
}

impl DedicatedDatabaseSpecification {
    /// Get slug
    pub fn slug(&self) -> &String {
        &self.slug
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get price
    pub fn price(&self) -> &f64 {
        &self.price
    }

    /// Get storage_overage_rate
    pub fn storage_overage_rate(&self) -> &f64 {
        &self.storage_overage_rate
    }

    /// Get bandwidth_overage_rate
    pub fn bandwidth_overage_rate(&self) -> &f64 {
        &self.bandwidth_overage_rate
    }

    /// Get replica_rate
    pub fn replica_rate(&self) -> &f64 {
        &self.replica_rate
    }

    /// Get pitr_rate
    pub fn pitr_rate(&self) -> &f64 {
        &self.pitr_rate
    }

    /// Get cpu
    pub fn cpu(&self) -> &i64 {
        &self.cpu
    }

    /// Get memory
    pub fn memory(&self) -> &i64 {
        &self.memory
    }

    /// Get max_connections
    pub fn max_connections(&self) -> &i64 {
        &self.max_connections
    }

    /// Get included_storage
    pub fn included_storage(&self) -> &i64 {
        &self.included_storage
    }

    /// Get included_bandwidth
    pub fn included_bandwidth(&self) -> &i64 {
        &self.included_bandwidth
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_specification_creation() {
        let _model = <DedicatedDatabaseSpecification as Default>::default();
        let _ = _model.slug();
        let _ = _model.name();
        let _ = _model.price();
        let _ = _model.storage_overage_rate();
        let _ = _model.bandwidth_overage_rate();
        let _ = _model.replica_rate();
        let _ = _model.pitr_rate();
        let _ = _model.cpu();
        let _ = _model.memory();
        let _ = _model.max_connections();
        let _ = _model.included_storage();
        let _ = _model.included_bandwidth();
        let _ = _model.enabled();
    }

    #[test]
    fn test_dedicated_database_specification_serialization() {
        let model = <DedicatedDatabaseSpecification as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseSpecification, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
