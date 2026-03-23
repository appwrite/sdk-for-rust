//! CountryList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Countries List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CountryList {
    /// Total number of countries that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of countries.
    #[serde(rename = "countries")]
    pub countries: Vec<crate::models::Country>,
}

impl CountryList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get countries
    pub fn countries(&self) -> &Vec<crate::models::Country> {
        &self.countries
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_list_creation() {
        let _model = <CountryList as Default>::default();
        let _ = _model.total();
        let _ = _model.countries();
    }

    #[test]
    fn test_country_list_serialization() {
        let model = <CountryList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<CountryList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
