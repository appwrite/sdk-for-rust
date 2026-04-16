//! BillingLimits model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// BillingLimits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingLimits {
    /// Bandwidth limit
    #[serde(rename = "bandwidth")]
    pub bandwidth: i64,
    /// Storage limit
    #[serde(rename = "storage")]
    pub storage: i64,
    /// Users limit
    #[serde(rename = "users")]
    pub users: i64,
    /// Executions limit
    #[serde(rename = "executions")]
    pub executions: i64,
    /// GBHours limit
    #[serde(rename = "GBHours")]
    pub gb_hours: i64,
    /// Image transformations limit
    #[serde(rename = "imageTransformations")]
    pub image_transformations: i64,
    /// Auth phone limit
    #[serde(rename = "authPhone")]
    pub auth_phone: i64,
    /// Budget limit percentage
    #[serde(rename = "budgetLimit")]
    pub budget_limit: i64,
}

impl BillingLimits {
    /// Get bandwidth
    pub fn bandwidth(&self) -> &i64 {
        &self.bandwidth
    }

    /// Get storage
    pub fn storage(&self) -> &i64 {
        &self.storage
    }

    /// Get users
    pub fn users(&self) -> &i64 {
        &self.users
    }

    /// Get executions
    pub fn executions(&self) -> &i64 {
        &self.executions
    }

    /// Get gb_hours
    pub fn gb_hours(&self) -> &i64 {
        &self.gb_hours
    }

    /// Get image_transformations
    pub fn image_transformations(&self) -> &i64 {
        &self.image_transformations
    }

    /// Get auth_phone
    pub fn auth_phone(&self) -> &i64 {
        &self.auth_phone
    }

    /// Get budget_limit
    pub fn budget_limit(&self) -> &i64 {
        &self.budget_limit
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_limits_creation() {
        let _model = <BillingLimits as Default>::default();
        let _ = _model.bandwidth();
        let _ = _model.storage();
        let _ = _model.users();
        let _ = _model.executions();
        let _ = _model.gb_hours();
        let _ = _model.image_transformations();
        let _ = _model.auth_phone();
        let _ = _model.budget_limit();
    }

    #[test]
    fn test_billing_limits_serialization() {
        let model = <BillingLimits as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingLimits, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
