//! DatabaseStatus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatus {
    /// Overall health status: healthy, degraded, unhealthy, or unknown when
    /// nothing could be measured.
    #[serde(rename = "health")]
    #[serde(default = "default_health")]
    pub health: String,
    /// Whether the database is ready to accept connections.
    #[serde(rename = "ready")]
    #[serde(default = "default_ready")]
    pub ready: bool,
    /// Database engine: postgresql, mysql, or mongodb.
    #[serde(rename = "engine")]
    #[serde(default = "default_engine")]
    pub engine: String,
    /// Database engine version.
    #[serde(rename = "version")]
    #[serde(default = "default_version")]
    pub version: String,
    /// Database uptime in seconds.
    #[serde(rename = "uptime")]
    #[serde(default = "default_uptime")]
    pub uptime: i64,
    /// Connection statistics. Null when the database has not been probed.
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<crate::models::DatabaseStatusConnections>,
    /// Requested replication sync mode. Possible values: async, sync, quorum.
    /// Compare with effectiveSyncMode for what the primary is enforcing.
    #[serde(rename = "syncMode")]
    #[serde(default = "default_sync_mode")]
    pub sync_mode: String,
    /// Replication sync mode the primary is actually enforcing. Null when high
    /// availability is disabled or the state could not be read.
    #[serde(rename = "effectiveSyncMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_sync_mode: Option<String>,
    /// Whether the enforced replication is weaker than the requested syncMode.
    #[serde(rename = "syncDegraded")]
    #[serde(default = "default_sync_degraded")]
    pub sync_degraded: bool,
    /// Number of standby acknowledgements the primary waits for before a write is
    /// committed.
    #[serde(rename = "syncAcknowledgements")]
    #[serde(default = "default_sync_acknowledgements")]
    pub sync_acknowledgements: i64,
    /// Number of standbys registered with the primary for synchronous replication.
    #[serde(rename = "syncStandbyCount")]
    #[serde(default = "default_sync_standby_count")]
    pub sync_standby_count: i64,
    /// Whether the other sync fields are an engine reading rather than a recorded
    /// estimate. True when the primary answered what it is enforcing, including
    /// when that answer contradicted the record, in which case the contradicted
    /// values are replaced by the ones the engine reports. False when the reading
    /// could not be taken: the probe did not answer, there was no engine to ask,
    /// or the values describe a configuration change just applied rather than
    /// anything measured. Absent when no engine was asked at all, so an unprobed
    /// database is distinguishable from an unconfirmed one. False never means a
    /// standby was found lagging, because it is the absence of a reading rather
    /// than a negative one, so draw no conclusion about replication health from it
    /// or from a response that omits it.
    #[serde(rename = "syncStateConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_state_confirmed: Option<bool>,
    /// List of database replicas and their status. Every configured member
    /// appears, including one the backend has not brought up, which is reported as
    /// not healthy.
    #[serde(rename = "replicas")]
    pub replicas: Vec<crate::models::DatabaseStatusReplica>,
    /// Storage volume information.
    #[serde(rename = "volumes")]
    pub volumes: Vec<crate::models::DatabaseStatusVolume>,
}

fn default_health() -> String {
    "unknown".to_string()
}

fn default_ready() -> bool {
    false
}

fn default_engine() -> String {
    "postgresql".to_string()
}

fn default_version() -> String {
    "".to_string()
}

fn default_uptime() -> i64 {
    0
}

fn default_sync_mode() -> String {
    "async".to_string()
}

fn default_sync_degraded() -> bool {
    false
}

fn default_sync_acknowledgements() -> i64 {
    0
}

fn default_sync_standby_count() -> i64 {
    0
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

    /// Set connections
    pub fn set_connections(
        mut self,
        connections: crate::models::DatabaseStatusConnections,
    ) -> Self {
        self.connections = Some(connections);
        self
    }

    /// Get connections
    pub fn connections(&self) -> Option<&crate::models::DatabaseStatusConnections> {
        self.connections.as_ref()
    }

    /// Get sync_mode
    pub fn sync_mode(&self) -> &String {
        &self.sync_mode
    }

    /// Set effective_sync_mode
    pub fn set_effective_sync_mode(mut self, effective_sync_mode: String) -> Self {
        self.effective_sync_mode = Some(effective_sync_mode);
        self
    }

    /// Get effective_sync_mode
    pub fn effective_sync_mode(&self) -> Option<&String> {
        self.effective_sync_mode.as_ref()
    }

    /// Get sync_degraded
    pub fn sync_degraded(&self) -> &bool {
        &self.sync_degraded
    }

    /// Get sync_acknowledgements
    pub fn sync_acknowledgements(&self) -> &i64 {
        &self.sync_acknowledgements
    }

    /// Get sync_standby_count
    pub fn sync_standby_count(&self) -> &i64 {
        &self.sync_standby_count
    }

    /// Set sync_state_confirmed
    pub fn set_sync_state_confirmed(mut self, sync_state_confirmed: bool) -> Self {
        self.sync_state_confirmed = Some(sync_state_confirmed);
        self
    }

    /// Get sync_state_confirmed
    pub fn sync_state_confirmed(&self) -> Option<&bool> {
        self.sync_state_confirmed.as_ref()
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
        let _ = _model.sync_mode();
        let _ = _model.sync_degraded();
        let _ = _model.sync_acknowledgements();
        let _ = _model.sync_standby_count();
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
