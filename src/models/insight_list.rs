//! InsightList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Insights List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct InsightList {
    /// Total number of insights that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of insights.
    #[serde(rename = "insights")]
    pub insights: Vec<crate::models::Insight>,
}

impl InsightList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get insights
    pub fn insights(&self) -> &Vec<crate::models::Insight> {
        &self.insights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insight_list_creation() {
        let _model = <InsightList as Default>::default();
        let _ = _model.total();
        let _ = _model.insights();
    }

    #[test]
    fn test_insight_list_serialization() {
        let model = <InsightList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<InsightList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
