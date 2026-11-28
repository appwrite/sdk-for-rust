//! AnalyticsMetricList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AnalyticsMetricList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsMetricList {
    /// Total number of metric rows returned.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// Metric rows: one per time bucket, one per dimension value, or a single row
    /// for the flat aggregate.
    #[serde(rename = "metrics")]
    #[serde(default = "default_metrics")]
    pub metrics: Vec<crate::models::AnalyticsMetric>,
}

fn default_total() -> i64 {
    0
}

fn default_metrics() -> Vec<crate::models::AnalyticsMetric> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl AnalyticsMetricList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get metrics
    pub fn metrics(&self) -> &Vec<crate::models::AnalyticsMetric> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_metric_list_creation() {
        let _model = <AnalyticsMetricList as Default>::default();
        let _ = _model.total();
        let _ = _model.metrics();
    }

    #[test]
    fn test_analytics_metric_list_serialization() {
        let model = <AnalyticsMetricList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsMetricList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
