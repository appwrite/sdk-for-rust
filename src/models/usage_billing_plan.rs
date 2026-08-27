//! UsageBillingPlan model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageBillingPlan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageBillingPlan {
    /// Bandwidth additional resources
    #[serde(rename = "bandwidth")]
    pub bandwidth: crate::models::AdditionalResource,
    /// Executions additional resources
    #[serde(rename = "executions")]
    pub executions: crate::models::AdditionalResource,
    /// Member additional resources
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::models::AdditionalResource>,
    /// Realtime additional resources
    #[serde(rename = "realtime")]
    pub realtime: crate::models::AdditionalResource,
    /// Realtime messages additional resources
    #[serde(rename = "realtimeMessages")]
    pub realtime_messages: crate::models::AdditionalResource,
    /// Realtime bandwidth additional resources
    #[serde(rename = "realtimeBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_bandwidth: Option<crate::models::AdditionalResource>,
    /// Storage additional resources
    #[serde(rename = "storage")]
    pub storage: crate::models::AdditionalResource,
    /// User additional resources
    #[serde(rename = "users")]
    pub users: crate::models::AdditionalResource,
    /// GBHour additional resources
    #[serde(rename = "GBHours")]
    pub gb_hours: crate::models::AdditionalResource,
    /// Image transformation additional resources
    #[serde(rename = "imageTransformations")]
    pub image_transformations: crate::models::AdditionalResource,
    /// Credits additional resources
    #[serde(rename = "credits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<crate::models::AdditionalResource>,
}

impl UsageBillingPlan {
    /// Get bandwidth
    pub fn bandwidth(&self) -> &crate::models::AdditionalResource {
        &self.bandwidth
    }

    /// Get executions
    pub fn executions(&self) -> &crate::models::AdditionalResource {
        &self.executions
    }

    /// Set member
    pub fn set_member(mut self, member: crate::models::AdditionalResource) -> Self {
        self.member = Some(member);
        self
    }

    /// Get member
    pub fn member(&self) -> Option<&crate::models::AdditionalResource> {
        self.member.as_ref()
    }

    /// Get realtime
    pub fn realtime(&self) -> &crate::models::AdditionalResource {
        &self.realtime
    }

    /// Get realtime_messages
    pub fn realtime_messages(&self) -> &crate::models::AdditionalResource {
        &self.realtime_messages
    }

    /// Set realtime_bandwidth
    pub fn set_realtime_bandwidth(
        mut self,
        realtime_bandwidth: crate::models::AdditionalResource,
    ) -> Self {
        self.realtime_bandwidth = Some(realtime_bandwidth);
        self
    }

    /// Get realtime_bandwidth
    pub fn realtime_bandwidth(&self) -> Option<&crate::models::AdditionalResource> {
        self.realtime_bandwidth.as_ref()
    }

    /// Get storage
    pub fn storage(&self) -> &crate::models::AdditionalResource {
        &self.storage
    }

    /// Get users
    pub fn users(&self) -> &crate::models::AdditionalResource {
        &self.users
    }

    /// Get gb_hours
    pub fn gb_hours(&self) -> &crate::models::AdditionalResource {
        &self.gb_hours
    }

    /// Get image_transformations
    pub fn image_transformations(&self) -> &crate::models::AdditionalResource {
        &self.image_transformations
    }

    /// Set credits
    pub fn set_credits(mut self, credits: crate::models::AdditionalResource) -> Self {
        self.credits = Some(credits);
        self
    }

    /// Get credits
    pub fn credits(&self) -> Option<&crate::models::AdditionalResource> {
        self.credits.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_billing_plan_creation() {
        let _model = <UsageBillingPlan as Default>::default();
        let _ = _model.bandwidth();
        let _ = _model.executions();
        let _ = _model.realtime();
        let _ = _model.realtime_messages();
        let _ = _model.storage();
        let _ = _model.users();
        let _ = _model.gb_hours();
        let _ = _model.image_transformations();
    }

    #[test]
    fn test_usage_billing_plan_serialization() {
        let model = <UsageBillingPlan as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageBillingPlan, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
