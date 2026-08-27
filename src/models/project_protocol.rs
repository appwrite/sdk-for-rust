//! ProjectProtocol model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ProjectProtocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProjectProtocol {
    /// Protocol ID.
    #[serde(rename = "$id")]
    pub id: crate::enums::ProjectProtocolId,
    /// Protocol status.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ProjectProtocol {
    /// Get id
    pub fn id(&self) -> &crate::enums::ProjectProtocolId {
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
    fn test_project_protocol_creation() {
        let _model = <ProjectProtocol as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_project_protocol_serialization() {
        let model = <ProjectProtocol as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProjectProtocol, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
