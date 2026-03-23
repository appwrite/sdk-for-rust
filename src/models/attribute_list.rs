//! AttributeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Attributes List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AttributeList {
    /// Total number of attributes in the given collection.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of attributes.
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
}

impl AttributeList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get attributes
    pub fn attributes(&self) -> &Vec<String> {
        &self.attributes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_list_creation() {
        let _model = <AttributeList as Default>::default();
        let _ = _model.total();
        let _ = _model.attributes();
    }

    #[test]
    fn test_attribute_list_serialization() {
        let model = <AttributeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AttributeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
