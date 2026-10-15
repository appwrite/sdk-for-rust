//! BillingPlanSupportedAddons model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// BillingPlanSupportedAddons
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanSupportedAddons {
    /// Whether the plan supports BAA (Business Associate Agreement) addon
    #[serde(rename = "baa")]
    #[serde(default = "default_baa")]
    pub baa: bool,
    /// Whether the plan supports Premium Geo DB addon (project-level)
    #[serde(rename = "premiumGeoDB")]
    #[serde(default = "default_premium_geo_db")]
    pub premium_geo_db: bool,
    /// Whether the plan supports Premium Geo DB addon (organization-level)
    #[serde(rename = "premiumGeoDBOrg")]
    #[serde(default = "default_premium_geo_db_org")]
    pub premium_geo_db_org: bool,
}

fn default_baa() -> bool {
    false
}

fn default_premium_geo_db() -> bool {
    false
}

fn default_premium_geo_db_org() -> bool {
    false
}

impl BillingPlanSupportedAddons {
    /// Get baa
    pub fn baa(&self) -> &bool {
        &self.baa
    }

    /// Get premium_geo_db
    pub fn premium_geo_db(&self) -> &bool {
        &self.premium_geo_db
    }

    /// Get premium_geo_db_org
    pub fn premium_geo_db_org(&self) -> &bool {
        &self.premium_geo_db_org
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_supported_addons_creation() {
        let _model = <BillingPlanSupportedAddons as Default>::default();
        let _ = _model.baa();
        let _ = _model.premium_geo_db();
        let _ = _model.premium_geo_db_org();
    }

    #[test]
    fn test_billing_plan_supported_addons_serialization() {
        let model = <BillingPlanSupportedAddons as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlanSupportedAddons, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
