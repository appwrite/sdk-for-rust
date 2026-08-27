//! ProjectAuthMethod model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ProjectAuthMethod
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProjectAuthMethod {
    /// Auth method ID.
    #[serde(rename = "$id")]
    pub id: crate::enums::ProjectAuthMethodId,
    /// Auth method status.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ProjectAuthMethod {
    /// Get id
    pub fn id(&self) -> &crate::enums::ProjectAuthMethodId {
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
    fn test_project_auth_method_creation() {
        let _model = <ProjectAuthMethod as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_project_auth_method_serialization() {
        let model = <ProjectAuthMethod as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProjectAuthMethod, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
