//! AppsList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Apps list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppsList {
    /// Total number of apps that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of apps.
    #[serde(rename = "apps")]
    pub apps: Vec<crate::models::App>,
}

impl AppsList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get apps
    pub fn apps(&self) -> &Vec<crate::models::App> {
        &self.apps
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apps_list_creation() {
        let _model = <AppsList as Default>::default();
        let _ = _model.total();
        let _ = _model.apps();
    }

    #[test]
    fn test_apps_list_serialization() {
        let model = <AppsList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppsList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
