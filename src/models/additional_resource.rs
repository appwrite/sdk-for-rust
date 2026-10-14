//! AdditionalResource model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AdditionalResource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AdditionalResource {
    /// Resource name
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Resource unit
    #[serde(rename = "unit")]
    #[serde(default = "default_unit")]
    pub unit: String,
    /// Price currency
    #[serde(rename = "currency")]
    #[serde(default = "default_currency")]
    pub currency: String,
    /// Price
    #[serde(rename = "price")]
    #[serde(default = "default_price")]
    pub price: f64,
    /// Resource value
    #[serde(rename = "value")]
    #[serde(default = "default_value")]
    pub value: i64,
    /// Description on invoice
    #[serde(rename = "invoiceDesc")]
    #[serde(default = "default_invoice_desc")]
    pub invoice_desc: String,
}

fn default_name() -> String {
    "".to_string()
}

fn default_unit() -> String {
    "".to_string()
}

fn default_currency() -> String {
    "".to_string()
}

fn default_price() -> f64 {
    0.0
}

fn default_value() -> i64 {
    0
}

fn default_invoice_desc() -> String {
    "".to_string()
}

impl AdditionalResource {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get unit
    pub fn unit(&self) -> &String {
        &self.unit
    }

    /// Get currency
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// Get price
    pub fn price(&self) -> &f64 {
        &self.price
    }

    /// Get value
    pub fn value(&self) -> &i64 {
        &self.value
    }

    /// Get invoice_desc
    pub fn invoice_desc(&self) -> &String {
        &self.invoice_desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_resource_creation() {
        let _model = <AdditionalResource as Default>::default();
        let _ = _model.name();
        let _ = _model.unit();
        let _ = _model.currency();
        let _ = _model.price();
        let _ = _model.value();
        let _ = _model.invoice_desc();
    }

    #[test]
    fn test_additional_resource_serialization() {
        let model = <AdditionalResource as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AdditionalResource, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
