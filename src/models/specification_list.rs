//! SpecificationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Specifications List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SpecificationList {
    /// Total number of specifications that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of specifications.
    #[serde(rename = "specifications")]
    pub specifications: Vec<crate::models::Specification>,
}

impl SpecificationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get specifications
    pub fn specifications(&self) -> &Vec<crate::models::Specification> {
        &self.specifications
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specification_list_creation() {
        let _model = <SpecificationList as Default>::default();
        let _ = _model.total();
        let _ = _model.specifications();
    }

    #[test]
    fn test_specification_list_serialization() {
        let model = <SpecificationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<SpecificationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
