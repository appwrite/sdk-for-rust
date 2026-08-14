//! BillingPlanAddon model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Addon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanAddon {
    /// Addon seats
    #[serde(rename = "seats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seats: Option<crate::models::BillingPlanAddonDetails>,
    /// Addon projects
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<crate::models::BillingPlanAddonDetails>,
}

impl BillingPlanAddon {
    /// Set seats
    pub fn set_seats(mut self, seats: crate::models::BillingPlanAddonDetails) -> Self {
        self.seats = Some(seats);
        self
    }

    /// Get seats
    pub fn seats(&self) -> Option<&crate::models::BillingPlanAddonDetails> {
        self.seats.as_ref()
    }

    /// Set projects
    pub fn set_projects(mut self, projects: crate::models::BillingPlanAddonDetails) -> Self {
        self.projects = Some(projects);
        self
    }

    /// Get projects
    pub fn projects(&self) -> Option<&crate::models::BillingPlanAddonDetails> {
        self.projects.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_addon_creation() {
        let _model = <BillingPlanAddon as Default>::default();
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
