//! CurrencyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Currencies List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CurrencyList {
    /// Total number of currencies that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of currencies.
    #[serde(rename = "currencies")]
    #[serde(default = "default_currencies")]
    pub currencies: Vec<crate::models::Currency>,
}

fn default_total() -> i64 {
    0
}

fn default_currencies() -> Vec<crate::models::Currency> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl CurrencyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get currencies
    pub fn currencies(&self) -> &Vec<crate::models::Currency> {
        &self.currencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_list_creation() {
        let _model = <CurrencyList as Default>::default();
        let _ = _model.total();
        let _ = _model.currencies();
    }

    #[test]
    fn test_currency_list_serialization() {
        let model = <CurrencyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<CurrencyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
