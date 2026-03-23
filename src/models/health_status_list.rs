//! HealthStatusList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Status List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthStatusList {
    /// Total number of statuses that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of statuses.
    #[serde(rename = "statuses")]
    pub statuses: Vec<crate::models::HealthStatus>,
}

impl HealthStatusList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get statuses
    pub fn statuses(&self) -> &Vec<crate::models::HealthStatus> {
        &self.statuses
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_list_creation() {
        let _model = <HealthStatusList as Default>::default();
        let _ = _model.total();
        let _ = _model.statuses();
    }

    #[test]
    fn test_health_status_list_serialization() {
        let model = <HealthStatusList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthStatusList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
