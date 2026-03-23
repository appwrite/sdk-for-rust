//! HealthStatus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthStatus {
    /// Name of the service.
    #[serde(rename = "name")]
    pub name: String,
    /// Duration in milliseconds how long the health check took.
    #[serde(rename = "ping")]
    pub ping: i64,
    /// Service status. Possible values are: `pass`, `fail`
    #[serde(rename = "status")]
    pub status: crate::enums::HealthCheckStatus,
}

impl HealthStatus {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get ping
    pub fn ping(&self) -> &i64 {
        &self.ping
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::HealthCheckStatus {
        &self.status
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_creation() {
        let _model = <HealthStatus as Default>::default();
        let _ = _model.name();
        let _ = _model.ping();
        let _ = _model.status();
    }

    #[test]
    fn test_health_status_serialization() {
        let model = <HealthStatus as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthStatus, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
