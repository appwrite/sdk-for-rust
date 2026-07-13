use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BillingPlanGroup {
    #[serde(rename = "starter")]
    #[default]
    Starter,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "scale")]
    Scale,
}

impl BillingPlanGroup {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            BillingPlanGroup::Starter => "starter",
            BillingPlanGroup::Pro => "pro",
            BillingPlanGroup::Scale => "scale",
        }
    }
}

impl std::fmt::Display for BillingPlanGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
