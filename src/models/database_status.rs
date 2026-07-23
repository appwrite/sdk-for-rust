//! DatabaseStatus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatus {
    /// Overall health status: healthy, degraded, or unhealthy.
    #[serde(rename = "health")]
    pub health: String,
    /// Whether the database is ready to accept connections.
    #[serde(rename = "ready")]
    pub ready: bool,
    /// Database engine: postgresql, mysql, mariadb, or mongodb.
    #[serde(rename = "engine")]
    pub engine: String,
    /// Database engine version.
    #[serde(rename = "version")]
    pub version: String,
    /// Database uptime in seconds.
    #[serde(rename = "uptime")]
    pub uptime: i64,
    /// Connection statistics.
    #[serde(rename = "connections")]
    pub connections: crate::models::DatabaseStatusConnections,
    /// List of database replicas and their status.
    #[serde(rename = "replicas")]
    pub replicas: Vec<crate::models::DatabaseStatusReplica>,
    /// Storage volume information.
    #[serde(rename = "volumes")]
    pub volumes: Vec<crate::models::DatabaseStatusVolume>,
}

impl DatabaseStatus {
    /// Get health
    pub fn health(&self) -> &String {
        &self.health
    }

    /// Get ready
    pub fn ready(&self) -> &bool {
        &self.ready
    }

    /// Get engine
    pub fn engine(&self) -> &String {
        &self.engine
    }

    /// Get version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get uptime
    pub fn uptime(&self) -> &i64 {
        &self.uptime
    }

    /// Get connections
    pub fn connections(&self) -> &crate::models::DatabaseStatusConnections {
        &self.connections
    }

    /// Get replicas
    pub fn replicas(&self) -> &Vec<crate::models::DatabaseStatusReplica> {
        &self.replicas
    }

    /// Get volumes
    pub fn volumes(&self) -> &Vec<crate::models::DatabaseStatusVolume> {
        &self.volumes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_status_creation() {
        let _model = <DatabaseStatus as Default>::default();
        let _ = _model.health();
        let _ = _model.ready();
        let _ = _model.engine();
        let _ = _model.version();
        let _ = _model.uptime();
        let _ = _model.connections();
        let _ = _model.replicas();
        let _ = _model.volumes();
    }

    #[test]
    fn test_database_status_serialization() {
        let model = <DatabaseStatus as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseStatus, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
