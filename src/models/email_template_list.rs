//! EmailTemplateList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Email Templates List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EmailTemplateList {
    /// Total number of templates that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of templates.
    #[serde(rename = "templates")]
    pub templates: Vec<crate::models::EmailTemplate>,
}

impl EmailTemplateList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get templates
    pub fn templates(&self) -> &Vec<crate::models::EmailTemplate> {
        &self.templates
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_template_list_creation() {
        let _model = <EmailTemplateList as Default>::default();
        let _ = _model.total();
        let _ = _model.templates();
    }

    #[test]
    fn test_email_template_list_serialization() {
        let model = <EmailTemplateList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EmailTemplateList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
