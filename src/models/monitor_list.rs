//! MonitorList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Monitors List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MonitorList {
    /// Total number of monitors that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of monitors.
    #[serde(rename = "monitors")]
    #[serde(default = "default_monitors")]
    pub monitors: Vec<crate::models::Monitor>,
}

fn default_total() -> i64 {
    0
}

fn default_monitors() -> Vec<crate::models::Monitor> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl MonitorList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get monitors
    pub fn monitors(&self) -> &Vec<crate::models::Monitor> {
        &self.monitors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_list_creation() {
        let _model = <MonitorList as Default>::default();
        let _ = _model.total();
        let _ = _model.monitors();
    }

    #[test]
    fn test_monitor_list_serialization() {
        let model = <MonitorList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MonitorList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
