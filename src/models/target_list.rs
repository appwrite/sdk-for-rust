//! TargetList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Target list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TargetList {
    /// Total number of targets that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of targets.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::Target>,
}

impl TargetList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get targets
    pub fn targets(&self) -> &Vec<crate::models::Target> {
        &self.targets
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_list_creation() {
        let _model = <TargetList as Default>::default();
        let _ = _model.total();
        let _ = _model.targets();
    }

    #[test]
    fn test_target_list_serialization() {
        let model = <TargetList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<TargetList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
