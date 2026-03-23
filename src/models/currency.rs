//! Currency model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Currency {
    /// Currency symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Currency name.
    #[serde(rename = "name")]
    pub name: String,
    /// Currency native symbol.
    #[serde(rename = "symbolNative")]
    pub symbol_native: String,
    /// Number of decimal digits.
    #[serde(rename = "decimalDigits")]
    pub decimal_digits: i64,
    /// Currency digit rounding.
    #[serde(rename = "rounding")]
    pub rounding: f64,
    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217)
    /// three-character format.
    #[serde(rename = "code")]
    pub code: String,
    /// Currency plural name
    #[serde(rename = "namePlural")]
    pub name_plural: String,
}

impl Currency {
    /// Get symbol
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get symbol_native
    pub fn symbol_native(&self) -> &String {
        &self.symbol_native
    }

    /// Get decimal_digits
    pub fn decimal_digits(&self) -> &i64 {
        &self.decimal_digits
    }

    /// Get rounding
    pub fn rounding(&self) -> &f64 {
        &self.rounding
    }

    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Get name_plural
    pub fn name_plural(&self) -> &String {
        &self.name_plural
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        let _model = <Currency as Default>::default();
        let _ = _model.symbol();
        let _ = _model.name();
        let _ = _model.symbol_native();
        let _ = _model.decimal_digits();
        let _ = _model.rounding();
        let _ = _model.code();
        let _ = _model.name_plural();
    }

    #[test]
    fn test_currency_serialization() {
        let model = <Currency as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Currency, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
