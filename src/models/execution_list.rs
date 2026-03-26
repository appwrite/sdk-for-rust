//! ExecutionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Executions List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ExecutionList {
    /// Total number of executions that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of executions.
    #[serde(rename = "executions")]
    pub executions: Vec<crate::models::Execution>,
}

impl ExecutionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get executions
    pub fn executions(&self) -> &Vec<crate::models::Execution> {
        &self.executions
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_list_creation() {
        let _model = <ExecutionList as Default>::default();
        let _ = _model.total();
        let _ = _model.executions();
    }

    #[test]
    fn test_execution_list_serialization() {
        let model = <ExecutionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ExecutionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
