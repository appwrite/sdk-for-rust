//! FunctionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Functions List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct FunctionList {
    /// Total number of functions that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of functions.
    #[serde(rename = "functions")]
    pub functions: Vec<crate::models::Function>,
}

impl FunctionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get functions
    pub fn functions(&self) -> &Vec<crate::models::Function> {
        &self.functions
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_list_creation() {
        let _model = <FunctionList as Default>::default();
        let _ = _model.total();
        let _ = _model.functions();
    }

    #[test]
    fn test_function_list_serialization() {
        let model = <FunctionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<FunctionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
