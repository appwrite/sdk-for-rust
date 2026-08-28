//! Program model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Program
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Program {
    /// Program ID
    #[serde(rename = "$id")]
    pub id: String,
    /// Program title
    #[serde(rename = "title")]
    pub title: String,
    /// Program description
    #[serde(rename = "description")]
    pub description: String,
    /// Program tag for highlighting on console
    #[serde(rename = "tag")]
    pub tag: String,
    /// Program icon for highlighting on console
    #[serde(rename = "icon")]
    pub icon: String,
    /// URL for more information on this program
    #[serde(rename = "url")]
    pub url: String,
    /// Whether this program is active
    #[serde(rename = "active")]
    pub active: bool,
    /// Whether this program is external
    #[serde(rename = "external")]
    pub external: bool,
    /// Billing plan ID that this is program is associated with.
    #[serde(rename = "billingPlanId")]
    pub billing_plan_id: String,
}

impl Program {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get title
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get tag
    pub fn tag(&self) -> &String {
        &self.tag
    }

    /// Get icon
    pub fn icon(&self) -> &String {
        &self.icon
    }

    /// Get url
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Get active
    pub fn active(&self) -> &bool {
        &self.active
    }

    /// Get external
    pub fn external(&self) -> &bool {
        &self.external
    }

    /// Get billing_plan_id
    pub fn billing_plan_id(&self) -> &String {
        &self.billing_plan_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_creation() {
        let _model = <Program as Default>::default();
        let _ = _model.id();
        let _ = _model.title();
        let _ = _model.description();
        let _ = _model.tag();
        let _ = _model.icon();
        let _ = _model.url();
        let _ = _model.active();
        let _ = _model.external();
        let _ = _model.billing_plan_id();
    }

    #[test]
    fn test_program_serialization() {
        let model = <Program as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Program, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
