//! AnalyticsAnnotation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AnalyticsAnnotation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsAnnotation {
    /// Annotation ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Annotation creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Annotation update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// ID of the analytics property the annotation belongs to.
    #[serde(rename = "propertyId")]
    #[serde(default = "default_property_id")]
    pub property_id: String,
    /// Point in time the annotation marks, in ISO 8601 format.
    #[serde(rename = "date")]
    #[serde(default = "default_date")]
    pub date: String,
    /// Annotation text.
    #[serde(rename = "content")]
    #[serde(default = "default_content")]
    pub content: String,
    /// ID of the user who created the annotation. Empty when it was created with
    /// an API key.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_property_id() -> String {
    "".to_string()
}

fn default_date() -> String {
    "".to_string()
}

fn default_content() -> String {
    "".to_string()
}

fn default_user_id() -> String {
    "".to_string()
}

impl AnalyticsAnnotation {
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

    /// Get property_id
    pub fn property_id(&self) -> &String {
        &self.property_id
    }

    /// Get date
    pub fn date(&self) -> &String {
        &self.date
    }

    /// Get content
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_annotation_creation() {
        let _model = <AnalyticsAnnotation as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.property_id();
        let _ = _model.date();
        let _ = _model.content();
        let _ = _model.user_id();
    }

    #[test]
    fn test_analytics_annotation_serialization() {
        let model = <AnalyticsAnnotation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsAnnotation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
