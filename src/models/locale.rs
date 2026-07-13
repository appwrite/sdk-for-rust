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
    /// City
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Name of timezone
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Postal code
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Latitude
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Longitude
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Autonomous System Number (ASN) of the IP
    #[serde(rename = "autonomousSystemNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_system_number: Option<String>,
    /// Organization that owns the ASN
    #[serde(rename = "autonomousSystemOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_system_organization: Option<String>,
    /// Internet service provider of the IP
    #[serde(rename = "isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    /// Connection type of the IP (e.g. cable, cellular, corporate)
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// User type classification of the IP (e.g. residential, business, hosting)
    #[serde(rename = "connectionUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_usage_type: Option<String>,
    /// Registered organization of the IP
    #[serde(rename = "connectionOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_organization: Option<String>,
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

    /// Set city
    pub fn set_city(mut self, city: String) -> Self {
        self.city = Some(city);
        self
    }

    /// Get city
    pub fn city(&self) -> Option<&String> {
        self.city.as_ref()
    }

    /// Set time_zone
    pub fn set_time_zone(mut self, time_zone: String) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    /// Get time_zone
    pub fn time_zone(&self) -> Option<&String> {
        self.time_zone.as_ref()
    }

    /// Set postal_code
    pub fn set_postal_code(mut self, postal_code: String) -> Self {
        self.postal_code = Some(postal_code);
        self
    }

    /// Get postal_code
    pub fn postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }

    /// Set latitude
    pub fn set_latitude(mut self, latitude: f64) -> Self {
        self.latitude = Some(latitude);
        self
    }

    /// Get latitude
    pub fn latitude(&self) -> Option<&f64> {
        self.latitude.as_ref()
    }

    /// Set longitude
    pub fn set_longitude(mut self, longitude: f64) -> Self {
        self.longitude = Some(longitude);
        self
    }

    /// Get longitude
    pub fn longitude(&self) -> Option<&f64> {
        self.longitude.as_ref()
    }

    /// Set autonomous_system_number
    pub fn set_autonomous_system_number(mut self, autonomous_system_number: String) -> Self {
        self.autonomous_system_number = Some(autonomous_system_number);
        self
    }

    /// Get autonomous_system_number
    pub fn autonomous_system_number(&self) -> Option<&String> {
        self.autonomous_system_number.as_ref()
    }

    /// Set autonomous_system_organization
    pub fn set_autonomous_system_organization(mut self, autonomous_system_organization: String) -> Self {
        self.autonomous_system_organization = Some(autonomous_system_organization);
        self
    }

    /// Get autonomous_system_organization
    pub fn autonomous_system_organization(&self) -> Option<&String> {
        self.autonomous_system_organization.as_ref()
    }

    /// Set isp
    pub fn set_isp(mut self, isp: String) -> Self {
        self.isp = Some(isp);
        self
    }

    /// Get isp
    pub fn isp(&self) -> Option<&String> {
        self.isp.as_ref()
    }

    /// Set connection_type
    pub fn set_connection_type(mut self, connection_type: String) -> Self {
        self.connection_type = Some(connection_type);
        self
    }

    /// Get connection_type
    pub fn connection_type(&self) -> Option<&String> {
        self.connection_type.as_ref()
    }

    /// Set connection_usage_type
    pub fn set_connection_usage_type(mut self, connection_usage_type: String) -> Self {
        self.connection_usage_type = Some(connection_usage_type);
        self
    }

    /// Get connection_usage_type
    pub fn connection_usage_type(&self) -> Option<&String> {
        self.connection_usage_type.as_ref()
    }

    /// Set connection_organization
    pub fn set_connection_organization(mut self, connection_organization: String) -> Self {
        self.connection_organization = Some(connection_organization);
        self
    }

    /// Get connection_organization
    pub fn connection_organization(&self) -> Option<&String> {
        self.connection_organization.as_ref()
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
