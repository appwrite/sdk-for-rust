//! BillingPlanAddonDetails model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanAddonDetails {
    /// Is the addon supported in the plan?
    #[serde(rename = "supported")]
    pub supported: bool,
    /// Addon plan included value
    #[serde(rename = "planIncluded")]
    pub plan_included: i64,
    /// Addon limit
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Addon type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Price currency
    #[serde(rename = "currency")]
    pub currency: String,
    /// Price
    #[serde(rename = "price")]
    pub price: f64,
    /// Resource value
    #[serde(rename = "value")]
    pub value: i64,
    /// Description on invoice
    #[serde(rename = "invoiceDesc")]
    pub invoice_desc: String,
}

impl BillingPlanAddonDetails {
    /// Get supported
    pub fn supported(&self) -> &bool {
        &self.supported
    }

    /// Get plan_included
    pub fn plan_included(&self) -> &i64 {
        &self.plan_included
    }

    /// Get limit
    pub fn limit(&self) -> &i64 {
        &self.limit
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
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
    fn test_billing_plan_addon_details_creation() {
        let _model = <BillingPlanAddonDetails as Default>::default();
        let _ = _model.supported();
        let _ = _model.plan_included();
        let _ = _model.limit();
        let _ = _model.r#type();
        let _ = _model.currency();
        let _ = _model.price();
        let _ = _model.value();
        let _ = _model.invoice_desc();
    }

    #[test]
    fn test_billing_plan_addon_details_serialization() {
        let model = <BillingPlanAddonDetails as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlanAddonDetails, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
