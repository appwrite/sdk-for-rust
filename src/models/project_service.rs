//! ProjectService model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ProjectService
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProjectService {
    /// Service ID.
    #[serde(rename = "$id")]
    pub id: crate::enums::ProjectServiceId,
    /// Service status.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ProjectService {
    /// Get id
    pub fn id(&self) -> &crate::enums::ProjectServiceId {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_service_creation() {
        let _model = <ProjectService as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_project_service_serialization() {
        let model = <ProjectService as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProjectService, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
