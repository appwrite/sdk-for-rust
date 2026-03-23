//! Country model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Country {
    /// Country name.
    #[serde(rename = "name")]
    pub name: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "code")]
    pub code: String,
}

impl Country {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_creation() {
        let _model = <Country as Default>::default();
        let _ = _model.name();
        let _ = _model.code();
    }

    #[test]
    fn test_country_serialization() {
        let model = <Country as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Country, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
