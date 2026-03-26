//! Locale model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Locale
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Locale {
    /// User IP address.
    #[serde(rename = "ip")]
    pub ip: String,
    /// Country code in [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1)
    /// two-character format
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Country name. This field support localization.
    #[serde(rename = "country")]
    pub country: String,
    /// Continent code. A two character continent code "AF" for Africa, "AN" for
    /// Antarctica, "AS" for Asia, "EU" for Europe, "NA" for North America, "OC"
    /// for Oceania, and "SA" for South America.
    #[serde(rename = "continentCode")]
    pub continent_code: String,
    /// Continent name. This field support localization.
    #[serde(rename = "continent")]
    pub continent: String,
    /// True if country is part of the European Union.
    #[serde(rename = "eu")]
    pub eu: bool,
    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217)
    /// three-character format
    #[serde(rename = "currency")]
    pub currency: String,
}

impl Locale {
    /// Get ip
    pub fn ip(&self) -> &String {
        &self.ip
    }

    /// Get country_code
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Get country
    pub fn country(&self) -> &String {
        &self.country
    }

    /// Get continent_code
    pub fn continent_code(&self) -> &String {
        &self.continent_code
    }

    /// Get continent
    pub fn continent(&self) -> &String {
        &self.continent
    }

    /// Get eu
    pub fn eu(&self) -> &bool {
        &self.eu
    }

    /// Get currency
    pub fn currency(&self) -> &String {
        &self.currency
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_creation() {
        let _model = <Locale as Default>::default();
        let _ = _model.ip();
        let _ = _model.country_code();
        let _ = _model.country();
        let _ = _model.continent_code();
        let _ = _model.continent();
        let _ = _model.eu();
        let _ = _model.currency();
    }

    #[test]
    fn test_locale_serialization() {
        let model = <Locale as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Locale, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
