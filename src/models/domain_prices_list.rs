//! DomainPricesList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Domain prices list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DomainPricesList {
    /// Total number of prices that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of prices.
    #[serde(rename = "prices")]
    #[serde(default = "default_prices")]
    pub prices: Vec<crate::models::DomainPrice>,
}

fn default_total() -> i64 {
    0
}

fn default_prices() -> Vec<crate::models::DomainPrice> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl DomainPricesList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get prices
    pub fn prices(&self) -> &Vec<crate::models::DomainPrice> {
        &self.prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_prices_list_creation() {
        let _model = <DomainPricesList as Default>::default();
        let _ = _model.total();
        let _ = _model.prices();
    }

    #[test]
    fn test_domain_prices_list_serialization() {
        let model = <DomainPricesList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DomainPricesList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
