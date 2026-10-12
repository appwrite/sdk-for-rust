//! DedicatedDatabaseSpecificationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// SpecificationList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseSpecificationList {
    /// List of dedicated database specifications.
    #[serde(rename = "specifications")]
    pub specifications: Vec<crate::models::DedicatedDatabaseSpecification>,
    /// Total number of specifications.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
}

fn default_total() -> i64 {
    0
}

impl DedicatedDatabaseSpecificationList {
    /// Get specifications
    pub fn specifications(&self) -> &Vec<crate::models::DedicatedDatabaseSpecification> {
        &self.specifications
    }

    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_specification_list_creation() {
        let _model = <DedicatedDatabaseSpecificationList as Default>::default();
        let _ = _model.specifications();
        let _ = _model.total();
    }

    #[test]
    fn test_dedicated_database_specification_list_serialization() {
        let model = <DedicatedDatabaseSpecificationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseSpecificationList, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
