//! RuntimeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Runtimes List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct RuntimeList {
    /// Total number of runtimes that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of runtimes.
    #[serde(rename = "runtimes")]
    pub runtimes: Vec<crate::models::Runtime>,
}

impl RuntimeList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get runtimes
    pub fn runtimes(&self) -> &Vec<crate::models::Runtime> {
        &self.runtimes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_list_creation() {
        let _model = <RuntimeList as Default>::default();
        let _ = _model.total();
        let _ = _model.runtimes();
    }

    #[test]
    fn test_runtime_list_serialization() {
        let model = <RuntimeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<RuntimeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
