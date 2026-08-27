//! BillingLimits model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Limits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingLimits {
    /// Bandwidth limit
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// Storage limit
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<i64>,
    /// Users limit
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<i64>,
    /// Executions limit
    #[serde(rename = "executions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<i64>,
    /// GBHours limit
    #[serde(rename = "GBHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_hours: Option<i64>,
    /// Image transformations limit
    #[serde(rename = "imageTransformations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_transformations: Option<i64>,
    /// Auth phone limit
    #[serde(rename = "authPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_phone: Option<i64>,
    /// Budget limit percentage
    #[serde(rename = "budgetLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_limit: Option<i64>,
}

impl BillingLimits {
    /// Set bandwidth
    pub fn set_bandwidth(mut self, bandwidth: i64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Get bandwidth
    pub fn bandwidth(&self) -> Option<&i64> {
        self.bandwidth.as_ref()
    }

    /// Set storage
    pub fn set_storage(mut self, storage: i64) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Get storage
    pub fn storage(&self) -> Option<&i64> {
        self.storage.as_ref()
    }

    /// Set users
    pub fn set_users(mut self, users: i64) -> Self {
        self.users = Some(users);
        self
    }

    /// Get users
    pub fn users(&self) -> Option<&i64> {
        self.users.as_ref()
    }

    /// Set executions
    pub fn set_executions(mut self, executions: i64) -> Self {
        self.executions = Some(executions);
        self
    }

    /// Get executions
    pub fn executions(&self) -> Option<&i64> {
        self.executions.as_ref()
    }

    /// Set gb_hours
    pub fn set_gb_hours(mut self, gb_hours: i64) -> Self {
        self.gb_hours = Some(gb_hours);
        self
    }

    /// Get gb_hours
    pub fn gb_hours(&self) -> Option<&i64> {
        self.gb_hours.as_ref()
    }

    /// Set image_transformations
    pub fn set_image_transformations(mut self, image_transformations: i64) -> Self {
        self.image_transformations = Some(image_transformations);
        self
    }

    /// Get image_transformations
    pub fn image_transformations(&self) -> Option<&i64> {
        self.image_transformations.as_ref()
    }

    /// Set auth_phone
    pub fn set_auth_phone(mut self, auth_phone: i64) -> Self {
        self.auth_phone = Some(auth_phone);
        self
    }

    /// Get auth_phone
    pub fn auth_phone(&self) -> Option<&i64> {
        self.auth_phone.as_ref()
    }

    /// Set budget_limit
    pub fn set_budget_limit(mut self, budget_limit: i64) -> Self {
        self.budget_limit = Some(budget_limit);
        self
    }

    /// Get budget_limit
    pub fn budget_limit(&self) -> Option<&i64> {
        self.budget_limit.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_limits_creation() {
        let _model = <BillingLimits as Default>::default();
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
