//! Phone model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Phone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Phone {
    /// Phone code.
    #[serde(rename = "code")]
    pub code: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName")]
    pub country_name: String,
}

impl Phone {
    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Get country_code
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Get country_name
    pub fn country_name(&self) -> &String {
        &self.country_name
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_creation() {
        let _model = <Phone as Default>::default();
        let _ = _model.code();
        let _ = _model.country_code();
        let _ = _model.country_name();
    }

    #[test]
    fn test_phone_serialization() {
        let model = <Phone as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Phone, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
