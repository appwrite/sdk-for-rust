//! Report model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Report {
    /// Report ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Report creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Report update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// ID of the third-party app that submitted the report.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Analyzer that produced this report. e.g. lighthouse, audit,
    /// databaseAnalyzer.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Short, human-readable title for the report.
    #[serde(rename = "title")]
    pub title: String,
    /// Markdown summary describing the report.
    #[serde(rename = "summary")]
    pub summary: String,
    /// Plural noun describing what the report analyzes, e.g. databases, sites,
    /// urls.
    #[serde(rename = "targetType")]
    pub target_type: String,
    /// Free-form target identifier (URL for lighthouse, resource ID for db).
    #[serde(rename = "target")]
    pub target: String,
    /// Categories covered by the report, e.g. performance, accessibility.
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    /// Insights nested under this report.
    #[serde(rename = "insights")]
    pub insights: Vec<crate::models::Insight>,
    /// Time the report was analyzed in ISO 8601 format.
    #[serde(rename = "analyzedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
}

impl Report {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get app_id
    pub fn app_id(&self) -> &String {
        &self.app_id
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get title
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get summary
    pub fn summary(&self) -> &String {
        &self.summary
    }

    /// Get target_type
    pub fn target_type(&self) -> &String {
        &self.target_type
    }

    /// Get target
    pub fn target(&self) -> &String {
        &self.target
    }

    /// Get categories
    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    /// Get insights
    pub fn insights(&self) -> &Vec<crate::models::Insight> {
        &self.insights
    }

    /// Set analyzed_at
    pub fn set_analyzed_at(mut self, analyzed_at: String) -> Self {
        self.analyzed_at = Some(analyzed_at);
        self
    }

    /// Get analyzed_at
    pub fn analyzed_at(&self) -> Option<&String> {
        self.analyzed_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_creation() {
        let _model = <Report as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.app_id();
        let _ = _model.r#type();
        let _ = _model.title();
        let _ = _model.summary();
        let _ = _model.target_type();
        let _ = _model.target();
        let _ = _model.categories();
        let _ = _model.insights();
    }

    #[test]
    fn test_report_serialization() {
        let model = <Report as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Report, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
