//! AppInstallationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// App installations list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppInstallationList {
    /// Total number of installations that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of installations.
    #[serde(rename = "installations")]
    pub installations: Vec<crate::models::AppInstallation>,
}

impl AppInstallationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get installations
    pub fn installations(&self) -> &Vec<crate::models::AppInstallation> {
        &self.installations
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_installation_list_creation() {
        let _model = <AppInstallationList as Default>::default();
        let _ = _model.total();
        let _ = _model.installations();
    }

    #[test]
    fn test_app_installation_list_serialization() {
        let model = <AppInstallationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppInstallationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
