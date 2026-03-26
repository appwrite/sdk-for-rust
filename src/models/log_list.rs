//! LogList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Logs List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct LogList {
    /// Total number of logs that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of logs.
    #[serde(rename = "logs")]
    pub logs: Vec<crate::models::Log>,
}

impl LogList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get logs
    pub fn logs(&self) -> &Vec<crate::models::Log> {
        &self.logs
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_list_creation() {
        let _model = <LogList as Default>::default();
        let _ = _model.total();
        let _ = _model.logs();
    }

    #[test]
    fn test_log_list_serialization() {
        let model = <LogList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<LogList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
