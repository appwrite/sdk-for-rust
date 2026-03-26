//! HealthAntivirus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Health Antivirus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthAntivirus {
    /// Antivirus version.
    #[serde(rename = "version")]
    pub version: String,
    /// Antivirus status. Possible values are: `disabled`, `offline`, `online`
    #[serde(rename = "status")]
    pub status: crate::enums::HealthAntivirusStatus,
}

impl HealthAntivirus {
    /// Get version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::HealthAntivirusStatus {
        &self.status
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_antivirus_creation() {
        let _model = <HealthAntivirus as Default>::default();
        let _ = _model.version();
        let _ = _model.status();
    }

    #[test]
    fn test_health_antivirus_serialization() {
        let model = <HealthAntivirus as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthAntivirus, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
