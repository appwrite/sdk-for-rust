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

        let deserialized: Result<DedicatedDatabaseSpecification, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
