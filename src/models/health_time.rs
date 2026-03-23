//! HealthTime model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Health Time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthTime {
    /// Current unix timestamp on trustful remote server.
    #[serde(rename = "remoteTime")]
    pub remote_time: i64,
    /// Current unix timestamp of local server where Appwrite runs.
    #[serde(rename = "localTime")]
    pub local_time: i64,
    /// Difference of unix remote and local timestamps in milliseconds.
    #[serde(rename = "diff")]
    pub diff: i64,
}

impl HealthTime {
    /// Get remote_time
    pub fn remote_time(&self) -> &i64 {
        &self.remote_time
    }

    /// Get local_time
    pub fn local_time(&self) -> &i64 {
        &self.local_time
    }

    /// Get diff
    pub fn diff(&self) -> &i64 {
        &self.diff
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_time_creation() {
        let _model = <HealthTime as Default>::default();
        let _ = _model.remote_time();
        let _ = _model.local_time();
        let _ = _model.diff();
    }

    #[test]
    fn test_health_time_serialization() {
        let model = <HealthTime as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthTime, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
