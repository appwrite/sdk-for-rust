//! BillingPlanAddon model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Addon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanAddon {
    /// Addon seats
    #[serde(rename = "seats")]
    pub seats: crate::models::BillingPlanAddonDetails,
    /// Addon projects
    #[serde(rename = "projects")]
    pub projects: crate::models::BillingPlanAddonDetails,
}

impl BillingPlanAddon {
    /// Get seats
    pub fn seats(&self) -> &crate::models::BillingPlanAddonDetails {
        &self.seats
    }

    /// Get projects
    pub fn projects(&self) -> &crate::models::BillingPlanAddonDetails {
        &self.projects
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_addon_creation() {
        let _model = <BillingPlanAddon as Default>::default();
        let _ = _model.seats();
        let _ = _model.projects();
    }

    #[test]
    fn test_billing_plan_addon_serialization() {
        let model = <BillingPlanAddon as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlanAddon, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
