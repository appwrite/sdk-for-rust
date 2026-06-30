//! UsageMetric model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageMetric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageMetric {
    /// Metric key this series describes.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Data points for this metric, ordered by time ascending. With `interval`,
    /// each entry is one bucket; without, each entry is one row of the dimensional
    /// or aggregate breakdown.
    #[serde(rename = "points")]
    pub points: Vec<crate::models::UsageDataPoint>,
}

impl UsageMetric {
    /// Get metric
    pub fn metric(&self) -> &String {
        &self.metric
    }

    /// Get points
    pub fn points(&self) -> &Vec<crate::models::UsageDataPoint> {
        &self.points
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_metric_creation() {
        let _model = <UsageMetric as Default>::default();
        let _ = _model.metric();
        let _ = _model.points();
    }

    #[test]
    fn test_usage_metric_serialization() {
        let model = <UsageMetric as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageMetric, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
