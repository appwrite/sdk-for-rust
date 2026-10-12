//! DomainPrice model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// DomainPrice
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DomainPrice {
    /// Domain name.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// Top-level domain for the requested domain.
    #[serde(rename = "tld")]
    #[serde(default = "default_tld")]
    pub tld: String,
    /// Whether the domain is currently available for registration.
    #[serde(rename = "available")]
    #[serde(default = "default_available")]
    pub available: bool,
    /// Domain registration price. Null when the price could not be resolved, for
    /// example for an unsupported TLD.
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Price period in years.
    #[serde(rename = "periodYears")]
    #[serde(default = "default_period_years")]
    pub period_years: i64,
    /// Whether the domain is a premium domain.
    #[serde(rename = "premium")]
    #[serde(default = "default_premium")]
    pub premium: bool,
    /// Domain renewal price for the same period. Null when the domain was not
    /// priced or the registrar has no renewal price for it.
    #[serde(rename = "renewalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// Renewal price period in years.
    #[serde(rename = "renewalPeriodYears")]
    #[serde(default = "default_renewal_period_years")]
    pub renewal_period_years: i64,
}

fn default_domain() -> String {
    "".to_string()
}

fn default_tld() -> String {
    "".to_string()
}

fn default_available() -> bool {
    false
}

fn default_period_years() -> i64 {
    1
}

fn default_premium() -> bool {
    false
}

fn default_renewal_period_years() -> i64 {
    1
}

impl DomainPrice {
    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get tld
    pub fn tld(&self) -> &String {
        &self.tld
    }

    /// Get available
    pub fn available(&self) -> &bool {
        &self.available
    }

    /// Set price
    pub fn set_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Get price
    pub fn price(&self) -> Option<&f64> {
        self.price.as_ref()
    }

    /// Get period_years
    pub fn period_years(&self) -> &i64 {
        &self.period_years
    }

    /// Get premium
    pub fn premium(&self) -> &bool {
        &self.premium
    }

    /// Set renewal_price
    pub fn set_renewal_price(mut self, renewal_price: f64) -> Self {
        self.renewal_price = Some(renewal_price);
        self
    }

    /// Get renewal_price
    pub fn renewal_price(&self) -> Option<&f64> {
        self.renewal_price.as_ref()
    }

    /// Get renewal_period_years
    pub fn renewal_period_years(&self) -> &i64 {
        &self.renewal_period_years
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_price_creation() {
        let _model = <DomainPrice as Default>::default();
        let _ = _model.domain();
        let _ = _model.tld();
        let _ = _model.available();
        let _ = _model.period_years();
        let _ = _model.premium();
        let _ = _model.renewal_period_years();
    }

    #[test]
    fn test_domain_price_serialization() {
        let model = <DomainPrice as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DomainPrice, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
