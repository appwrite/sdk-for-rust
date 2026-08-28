//! DedicatedDatabaseReplicas model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Replicas
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseReplicas {
    /// Number of configured replicas. Zero means high availability is disabled.
    #[serde(rename = "replicas")]
    pub replicas: i64,
    /// Requested replication sync mode. Possible values: async (asynchronous,
    /// fastest), sync (synchronous, strong consistency), quorum (quorum-based,
    /// majority of replicas must confirm). This is what was asked for; compare it
    /// with effectiveSyncMode for what the primary is enforcing.
    #[serde(rename = "syncMode")]
    pub sync_mode: String,
    /// Replication sync mode the primary is actually enforcing. Null when high
    /// availability is disabled or the state could not be read. A value below the
    /// requested syncMode means writes are being acknowledged with weaker
    /// durability than configured.
    #[serde(rename = "effectiveSyncMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_sync_mode: Option<String>,
    /// Whether the enforced replication is weaker than the requested syncMode.
    #[serde(rename = "syncDegraded")]
    pub sync_degraded: bool,
    /// Number of standby acknowledgements the primary waits for before a write is
    /// committed. Zero means writes are acknowledged locally.
    #[serde(rename = "syncAcknowledgements")]
    pub sync_acknowledgements: i64,
    /// Number of standbys registered with the primary for synchronous replication.
    #[serde(rename = "syncStandbyCount")]
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
    /// Per-pod statuses for the primary and every replica.
    #[serde(rename = "members")]
    pub members: Vec<crate::models::DedicatedDatabaseMember>,
}

impl DedicatedDatabaseReplicas {
    /// Get replicas
    pub fn replicas(&self) -> &i64 {
        &self.replicas
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

    /// Get members
    pub fn members(&self) -> &Vec<crate::models::DedicatedDatabaseMember> {
        &self.members
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_replicas_creation() {
        let _model = <DedicatedDatabaseReplicas as Default>::default();
        let _ = _model.replicas();
        let _ = _model.sync_mode();
        let _ = _model.sync_degraded();
        let _ = _model.sync_acknowledgements();
        let _ = _model.sync_standby_count();
        let _ = _model.members();
    }

    #[test]
    fn test_dedicated_database_replicas_serialization() {
        let model = <DedicatedDatabaseReplicas as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseReplicas, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
