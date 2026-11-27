//! AnalyticsAnnotationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Analytics annotations list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsAnnotationList {
    /// Total number of annotations that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of annotations.
    #[serde(rename = "annotations")]
    #[serde(default = "default_annotations")]
    pub annotations: Vec<crate::models::AnalyticsAnnotation>,
}

fn default_total() -> i64 {
    0
}

fn default_annotations() -> Vec<crate::models::AnalyticsAnnotation> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl AnalyticsAnnotationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get annotations
    pub fn annotations(&self) -> &Vec<crate::models::AnalyticsAnnotation> {
        &self.annotations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_annotation_list_creation() {
        let _model = <AnalyticsAnnotationList as Default>::default();
        let _ = _model.total();
        let _ = _model.annotations();
    }

    #[test]
    fn test_analytics_annotation_list_serialization() {
        let model = <AnalyticsAnnotationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsAnnotationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
