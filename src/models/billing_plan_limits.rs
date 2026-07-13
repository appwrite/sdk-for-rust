//! BillingPlanLimits model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// PlanLimits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanLimits {
    /// Credits limit per billing cycle
    #[serde(rename = "credits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<i64>,
    /// Daily credits limit (if applicable)
    #[serde(rename = "dailyCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_credits: Option<i64>,
}

impl BillingPlanLimits {
    /// Set credits
    pub fn set_credits(mut self, credits: i64) -> Self {
        self.credits = Some(credits);
        self
    }

    /// Get credits
    pub fn credits(&self) -> Option<&i64> {
        self.credits.as_ref()
    }

    /// Set daily_credits
    pub fn set_daily_credits(mut self, daily_credits: i64) -> Self {
        self.daily_credits = Some(daily_credits);
        self
    }

    /// Get daily_credits
    pub fn daily_credits(&self) -> Option<&i64> {
        self.daily_credits.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_limits_creation() {
        let _model = <BillingPlanLimits as Default>::default();
    }

    #[test]
    fn test_billing_plan_limits_serialization() {
        let model = <BillingPlanLimits as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlanLimits, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
