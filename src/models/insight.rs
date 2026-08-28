//! Insight model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Insight
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Insight {
    /// Insight ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Insight creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Insight update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Parent report ID. Insights always belong to a report.
    #[serde(rename = "reportId")]
    pub report_id: String,
    /// Insight type. One of databaseIndex (legacy), tablesDBIndex,
    /// documentsDBIndex, vectorsDBIndex, databasePerformance, sitePerformance,
    /// siteAccessibility, siteSeo, functionPerformance. The index types are
    /// engine-specific so each CTA can pair the right service+method
    /// (databases.createIndex, tablesDB.createIndex, documentsDB.createIndex, or
    /// vectorsDB.createIndex).
    #[serde(rename = "type")]
    pub r#type: String,
    /// Insight severity. One of info, warning, critical.
    #[serde(rename = "severity")]
    pub severity: String,
    /// Insight status. One of active, dismissed.
    #[serde(rename = "status")]
    pub status: String,
    /// Type of the resource the insight is about. Plural noun, e.g. databases,
    /// sites, functions.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// ID of the resource the insight is about.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Plural noun for the parent resource that contains the insight's resource,
    /// e.g. an insight about a column index on a table → resourceType=indexes,
    /// parentResourceType=tables. Empty when the resource has no parent.
    #[serde(rename = "parentResourceType")]
    pub parent_resource_type: String,
    /// ID of the parent resource. Empty when the resource has no parent.
    #[serde(rename = "parentResourceId")]
    pub parent_resource_id: String,
    /// Insight title.
    #[serde(rename = "title")]
    pub title: String,
    /// Short markdown summary describing the insight.
    #[serde(rename = "summary")]
    pub summary: String,
    /// List of call-to-action buttons attached to this insight.
    #[serde(rename = "ctas")]
    pub ctas: Vec<crate::models::InsightCTA>,
    /// Time the insight was analyzed in ISO 8601 format.
    #[serde(rename = "analyzedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    /// Time the insight was dismissed in ISO 8601 format. Empty when not
    /// dismissed.
    #[serde(rename = "dismissedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_at: Option<String>,
    /// User ID that dismissed the insight. Empty when not dismissed.
    #[serde(rename = "dismissedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_by: Option<String>,
}

impl Insight {
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

    /// Get report_id
    pub fn report_id(&self) -> &String {
        &self.report_id
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get severity
    pub fn severity(&self) -> &String {
        &self.severity
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get parent_resource_type
    pub fn parent_resource_type(&self) -> &String {
        &self.parent_resource_type
    }

    /// Get parent_resource_id
    pub fn parent_resource_id(&self) -> &String {
        &self.parent_resource_id
    }

    /// Get title
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get summary
    pub fn summary(&self) -> &String {
        &self.summary
    }

    /// Get ctas
    pub fn ctas(&self) -> &Vec<crate::models::InsightCTA> {
        &self.ctas
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

    /// Set dismissed_at
    pub fn set_dismissed_at(mut self, dismissed_at: String) -> Self {
        self.dismissed_at = Some(dismissed_at);
        self
    }

    /// Get dismissed_at
    pub fn dismissed_at(&self) -> Option<&String> {
        self.dismissed_at.as_ref()
    }

    /// Set dismissed_by
    pub fn set_dismissed_by(mut self, dismissed_by: String) -> Self {
        self.dismissed_by = Some(dismissed_by);
        self
    }

    /// Get dismissed_by
    pub fn dismissed_by(&self) -> Option<&String> {
        self.dismissed_by.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insight_creation() {
        let _model = <Insight as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.report_id();
        let _ = _model.r#type();
        let _ = _model.severity();
        let _ = _model.status();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.parent_resource_type();
        let _ = _model.parent_resource_id();
        let _ = _model.title();
        let _ = _model.summary();
        let _ = _model.ctas();
    }

    #[test]
    fn test_insight_serialization() {
        let model = <Insight as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Insight, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
