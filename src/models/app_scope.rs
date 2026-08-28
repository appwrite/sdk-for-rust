//! AppScope model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AppScope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppScope {
    /// Scope value as requested by apps.
    #[serde(rename = "value")]
    pub value: String,
    /// Human-readable description of what the scope grants.
    #[serde(rename = "description")]
    pub description: String,
    /// What the scope grants access to. One of `account`, `project`, or
    /// `organization`. Only `project` and `organization` scopes are installable.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Scope category, used to group scopes on consent and installation screens.
    #[serde(rename = "category")]
    pub category: String,
    /// Whether the scope is deprecated. Deprecated scopes can still be requested
    /// but should not be offered for new grants.
    #[serde(rename = "deprecated")]
    pub deprecated: bool,
}

impl AppScope {
    /// Get value
    pub fn value(&self) -> &String {
        &self.value
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get category
    pub fn category(&self) -> &String {
        &self.category
    }

    /// Get deprecated
    pub fn deprecated(&self) -> &bool {
        &self.deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_scope_creation() {
        let _model = <AppScope as Default>::default();
        let _ = _model.value();
        let _ = _model.description();
        let _ = _model.r#type();
        let _ = _model.category();
        let _ = _model.deprecated();
    }

    #[test]
    fn test_app_scope_serialization() {
        let model = <AppScope as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppScope, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
