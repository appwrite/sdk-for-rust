//! DedicatedDatabaseSpecification model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseSpecification {
    /// Specification slug. Use this value when creating a dedicated database.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Human readable specification name.
    #[serde(rename = "name")]
    pub name: String,
    /// Monthly price of the specification in USD.
    #[serde(rename = "price")]
    pub price: f64,
    /// Price per GB of storage above the included amount, per month, in USD.
    #[serde(rename = "storageOverageRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_overage_rate: Option<f64>,
    /// Price per GB of bandwidth above the included amount, per month, in USD.
    #[serde(rename = "bandwidthOverageRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_overage_rate: Option<f64>,
    /// High availability replica price as a fraction of the specification price.
    #[serde(rename = "replicaRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_rate: Option<f64>,
    /// Point-in-time recovery price as a fraction of the specification price.
    #[serde(rename = "pitrRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitr_rate: Option<f64>,
    /// Allocated CPU in millicores.
    #[serde(rename = "cpu")]
    pub cpu: i64,
    /// Allocated memory in MB.
    #[serde(rename = "memory")]
    pub memory: i64,
    /// Maximum number of concurrent connections.
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
    /// Included storage in GB before overage charges apply.
    #[serde(rename = "includedStorage")]
    pub included_storage: i64,
    /// Included bandwidth in GB before overage charges apply.
    #[serde(rename = "includedBandwidth")]
    pub included_bandwidth: i64,
    /// Whether the specification is available on the current plan.
    #[serde(rename = "enabled")]
    pub enabled: bool,
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

    /// Set storage_overage_rate
    pub fn set_storage_overage_rate(mut self, storage_overage_rate: f64) -> Self {
        self.storage_overage_rate = Some(storage_overage_rate);
        self
    }

    /// Get storage_overage_rate
    pub fn storage_overage_rate(&self) -> Option<&f64> {
        self.storage_overage_rate.as_ref()
    }

    /// Set bandwidth_overage_rate
    pub fn set_bandwidth_overage_rate(mut self, bandwidth_overage_rate: f64) -> Self {
        self.bandwidth_overage_rate = Some(bandwidth_overage_rate);
        self
    }

    /// Get bandwidth_overage_rate
    pub fn bandwidth_overage_rate(&self) -> Option<&f64> {
        self.bandwidth_overage_rate.as_ref()
    }

    /// Set replica_rate
    pub fn set_replica_rate(mut self, replica_rate: f64) -> Self {
        self.replica_rate = Some(replica_rate);
        self
    }

    /// Get replica_rate
    pub fn replica_rate(&self) -> Option<&f64> {
        self.replica_rate.as_ref()
    }

    /// Set pitr_rate
    pub fn set_pitr_rate(mut self, pitr_rate: f64) -> Self {
        self.pitr_rate = Some(pitr_rate);
        self
    }

    /// Get pitr_rate
    pub fn pitr_rate(&self) -> Option<&f64> {
        self.pitr_rate.as_ref()
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
