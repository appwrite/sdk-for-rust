//! MockNumber model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Mock Number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MockNumber {
    /// Mock phone number for testing phone authentication. Useful for testing
    /// phone authentication without sending an SMS.
    #[serde(rename = "number")]
    pub number: String,
    /// Mock OTP for the number.
    #[serde(rename = "otp")]
    pub otp: String,
    /// Attribute creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Attribute update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
}

impl MockNumber {
    /// Get number
    pub fn number(&self) -> &String {
        &self.number
    }

    /// Get otp
    pub fn otp(&self) -> &String {
        &self.otp
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_number_creation() {
        let _model = <MockNumber as Default>::default();
        let _ = _model.number();
        let _ = _model.otp();
        let _ = _model.created_at();
        let _ = _model.updated_at();
    }

    #[test]
    fn test_mock_number_serialization() {
        let model = <MockNumber as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MockNumber, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
