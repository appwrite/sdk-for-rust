//! DedicatedDatabasePooler model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// PoolerConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabasePooler {
    /// Whether connection pooling is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Connection pool mode. Possible values: transaction (releases connections
    /// back to pool after each transaction), session (holds connections for the
    /// entire client session).
    #[serde(rename = "mode")]
    pub mode: String,
    /// Client-connection ceiling the pooler accepts. Enforced on MySQL and
    /// MariaDB; on PostgreSQL the pooler has no client cap, so this reports the
    /// database's advertised networkMaxConnections and cannot be set here.
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
    /// Default pool size per user.
    #[serde(rename = "defaultPoolSize")]
    pub default_pool_size: i64,
    /// Pooler listening port.
    #[serde(rename = "port")]
    pub port: i64,
    /// Whether SELECTs are routed to HA replicas while writes and locked reads
    /// stay on the primary. Active only when HA is enabled.
    #[serde(rename = "readWriteSplitting")]
    pub read_write_splitting: bool,
    /// Effective CPU request applied to the pooler sidecar container (Kubernetes
    /// quantity). Returns the proportional default (5% of DB CPU, floor 100m)
    /// unless overridden.
    #[serde(rename = "poolerCpuRequest")]
    pub pooler_cpu_request: String,
    /// Effective CPU limit applied to the pooler sidecar container (Kubernetes
    /// quantity). Returns the proportional default (10% of DB CPU, floor 200m)
    /// unless overridden.
    #[serde(rename = "poolerCpuLimit")]
    pub pooler_cpu_limit: String,
    /// Effective memory request applied to the pooler sidecar container
    /// (Kubernetes quantity). Returns the proportional default (7.5% of DB memory,
    /// floor 64Mi) unless overridden.
    #[serde(rename = "poolerMemoryRequest")]
    pub pooler_memory_request: String,
    /// Effective memory limit applied to the pooler sidecar container (Kubernetes
    /// quantity). Returns the proportional default (15% of DB memory, floor 128Mi)
    /// unless overridden.
    #[serde(rename = "poolerMemoryLimit")]
    pub pooler_memory_limit: String,
}

impl DedicatedDatabasePooler {
    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get mode
    pub fn mode(&self) -> &String {
        &self.mode
    }

    /// Get max_connections
    pub fn max_connections(&self) -> &i64 {
        &self.max_connections
    }

    /// Get default_pool_size
    pub fn default_pool_size(&self) -> &i64 {
        &self.default_pool_size
    }

    /// Get port
    pub fn port(&self) -> &i64 {
        &self.port
    }

    /// Get read_write_splitting
    pub fn read_write_splitting(&self) -> &bool {
        &self.read_write_splitting
    }

    /// Get pooler_cpu_request
    pub fn pooler_cpu_request(&self) -> &String {
        &self.pooler_cpu_request
    }

    /// Get pooler_cpu_limit
    pub fn pooler_cpu_limit(&self) -> &String {
        &self.pooler_cpu_limit
    }

    /// Get pooler_memory_request
    pub fn pooler_memory_request(&self) -> &String {
        &self.pooler_memory_request
    }

    /// Get pooler_memory_limit
    pub fn pooler_memory_limit(&self) -> &String {
        &self.pooler_memory_limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_pooler_creation() {
        let _model = <DedicatedDatabasePooler as Default>::default();
        let _ = _model.enabled();
        let _ = _model.mode();
        let _ = _model.max_connections();
        let _ = _model.default_pool_size();
        let _ = _model.port();
        let _ = _model.read_write_splitting();
        let _ = _model.pooler_cpu_request();
        let _ = _model.pooler_cpu_limit();
        let _ = _model.pooler_memory_request();
        let _ = _model.pooler_memory_limit();
    }

    #[test]
    fn test_dedicated_database_pooler_serialization() {
        let model = <DedicatedDatabasePooler as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabasePooler, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
