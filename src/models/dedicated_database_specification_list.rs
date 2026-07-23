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
    pub total: i64,
    /// Overage and add-on pricing shared across all specifications.
    #[serde(rename = "pricing")]
    pub pricing: crate::models::DedicatedDatabaseSpecificationPricing,
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

    /// Get pricing
    pub fn pricing(&self) -> &crate::models::DedicatedDatabaseSpecificationPricing {
        &self.pricing
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
        let _ = _model.pricing();
    }

    #[test]
    fn test_dedicated_database_specification_list_serialization() {
        let model = <DedicatedDatabaseSpecificationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseSpecificationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
