//! AlertList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Alerts List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlertList {
    /// Total number of alerts that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of alerts.
    #[serde(rename = "alerts")]
    #[serde(default = "default_alerts")]
    pub alerts: Vec<crate::models::Alert>,
}

fn default_total() -> i64 {
    0
}

fn default_alerts() -> Vec<crate::models::Alert> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl AlertList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get alerts
    pub fn alerts(&self) -> &Vec<crate::models::Alert> {
        &self.alerts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_list_creation() {
        let _model = <AlertList as Default>::default();
        let _ = _model.total();
        let _ = _model.alerts();
    }

    #[test]
    fn test_alert_list_serialization() {
        let model = <AlertList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlertList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
