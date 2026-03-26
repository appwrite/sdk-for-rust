//! FrameworkList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Frameworks List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct FrameworkList {
    /// Total number of frameworks that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of frameworks.
    #[serde(rename = "frameworks")]
    pub frameworks: Vec<crate::models::Framework>,
}

impl FrameworkList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get frameworks
    pub fn frameworks(&self) -> &Vec<crate::models::Framework> {
        &self.frameworks
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_list_creation() {
        let _model = <FrameworkList as Default>::default();
        let _ = _model.total();
        let _ = _model.frameworks();
    }

    #[test]
    fn test_framework_list_serialization() {
        let model = <FrameworkList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<FrameworkList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
