//! VariableList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Variables List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VariableList {
    /// Total number of variables that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of variables.
    #[serde(rename = "variables")]
    pub variables: Vec<crate::models::Variable>,
}

impl VariableList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get variables
    pub fn variables(&self) -> &Vec<crate::models::Variable> {
        &self.variables
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_list_creation() {
        let _model = <VariableList as Default>::default();
        let _ = _model.total();
        let _ = _model.variables();
    }

    #[test]
    fn test_variable_list_serialization() {
        let model = <VariableList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VariableList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
