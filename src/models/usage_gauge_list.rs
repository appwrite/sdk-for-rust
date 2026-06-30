//! UsageGaugeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageGaugeList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageGaugeList {
    /// Time interval size (1h or 1d). Empty when the request omits `interval` —
    /// points then carry the request end time as their as-of marker.
    #[serde(rename = "interval")]
    pub interval: String,
    /// One entry per requested metric, each carrying its own points[] time series
    /// (latest-snapshot per bucket / dimension via argMax over time).
    #[serde(rename = "metrics")]
    pub metrics: Vec<crate::models::UsageMetric>,
}

impl UsageGaugeList {
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
    fn test_usage_gauge_list_creation() {
        let _model = <UsageGaugeList as Default>::default();
        let _ = _model.interval();
        let _ = _model.metrics();
    }

    #[test]
    fn test_usage_gauge_list_serialization() {
        let model = <UsageGaugeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageGaugeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
