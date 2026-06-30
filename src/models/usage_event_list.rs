//! UsageEventList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageEventList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageEventList {
    /// Time interval size (1h or 1d). Empty when the request omits `interval` —
    /// points then carry the request end time as their as-of marker.
    #[serde(rename = "interval")]
    pub interval: String,
    /// One entry per requested metric, each carrying its own points[] time series
    /// (sums per bucket / dimension over time).
    #[serde(rename = "metrics")]
    pub metrics: Vec<crate::models::UsageMetric>,
}

impl UsageEventList {
    /// Get interval
    pub fn interval(&self) -> &String {
        &self.interval
    }

    /// Get metrics
    pub fn metrics(&self) -> &Vec<crate::models::UsageMetric> {
        &self.metrics
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_event_list_creation() {
        let _model = <UsageEventList as Default>::default();
        let _ = _model.interval();
        let _ = _model.metrics();
    }

    #[test]
    fn test_usage_event_list_serialization() {
        let model = <UsageEventList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageEventList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
