//! PlatformList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Platforms List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PlatformList {
    /// Total number of platforms in the given project.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of platforms.
    #[serde(rename = "platforms")]
    pub platforms: Vec<serde_json::Value>,
}

impl PlatformList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get platforms
    pub fn platforms(&self) -> &Vec<serde_json::Value> {
        &self.platforms
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_list_creation() {
        let _model = <PlatformList as Default>::default();
        let _ = _model.total();
        let _ = _model.platforms();
    }

    #[test]
    fn test_platform_list_serialization() {
        let model = <PlatformList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PlatformList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
