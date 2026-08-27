//! DedicatedDatabaseRestorationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Dedicated database restorations list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseRestorationList {
    /// Total number of restorations that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of restorations.
    #[serde(rename = "restorations")]
    pub restorations: Vec<crate::models::DedicatedDatabaseRestoration>,
}

impl DedicatedDatabaseRestorationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get restorations
    pub fn restorations(&self) -> &Vec<crate::models::DedicatedDatabaseRestoration> {
        &self.restorations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_restoration_list_creation() {
        let _model = <DedicatedDatabaseRestorationList as Default>::default();
        let _ = _model.total();
        let _ = _model.restorations();
    }

    #[test]
    fn test_dedicated_database_restoration_list_serialization() {
        let model = <DedicatedDatabaseRestorationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseRestorationList, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
