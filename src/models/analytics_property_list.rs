//! AnalyticsPropertyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Analytics properties list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsPropertyList {
    /// Total number of properties that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of properties.
    #[serde(rename = "properties")]
    #[serde(default = "default_properties")]
    pub properties: Vec<crate::models::AnalyticsProperty>,
}

fn default_total() -> i64 {
    0
}

fn default_properties() -> Vec<crate::models::AnalyticsProperty> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl AnalyticsPropertyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get properties
    pub fn properties(&self) -> &Vec<crate::models::AnalyticsProperty> {
        &self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_property_list_creation() {
        let _model = <AnalyticsPropertyList as Default>::default();
        let _ = _model.total();
        let _ = _model.properties();
    }

    #[test]
    fn test_analytics_property_list_serialization() {
        let model = <AnalyticsPropertyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsPropertyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
