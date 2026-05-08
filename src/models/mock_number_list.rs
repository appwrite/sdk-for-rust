//! MockNumberList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Mock Numbers List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MockNumberList {
    /// Total number of mockNumbers that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of mockNumbers.
    #[serde(rename = "mockNumbers")]
    pub mock_numbers: Vec<crate::models::MockNumber>,
}

impl MockNumberList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get mock_numbers
    pub fn mock_numbers(&self) -> &Vec<crate::models::MockNumber> {
        &self.mock_numbers
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_number_list_creation() {
        let _model = <MockNumberList as Default>::default();
        let _ = _model.total();
        let _ = _model.mock_numbers();
    }

    #[test]
    fn test_mock_number_list_serialization() {
        let model = <MockNumberList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MockNumberList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
