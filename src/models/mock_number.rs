//! MockNumber model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Mock Number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MockNumber {
    /// Mock phone number for testing phone authentication. Useful for testing
    /// phone authentication without sending an SMS.
    #[serde(rename = "phone")]
    pub phone: String,
    /// Mock OTP for the number.
    #[serde(rename = "otp")]
    pub otp: String,
}

impl MockNumber {
    /// Get phone
    pub fn phone(&self) -> &String {
        &self.phone
    }

    /// Get otp
    pub fn otp(&self) -> &String {
        &self.otp
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_number_creation() {
        let _model = <MockNumber as Default>::default();
        let _ = _model.phone();
        let _ = _model.otp();
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
