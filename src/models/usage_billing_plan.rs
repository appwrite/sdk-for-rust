//! UsageBillingPlan model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageBillingPlan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageBillingPlan {
    /// Bandwidth additional resources
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<crate::models::AdditionalResource>,
    /// Executions additional resources
    #[serde(rename = "executions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<crate::models::AdditionalResource>,
    /// Member additional resources
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::models::AdditionalResource>,
    /// Realtime additional resources
    #[serde(rename = "realtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime: Option<crate::models::AdditionalResource>,
    /// Realtime messages additional resources
    #[serde(rename = "realtimeMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_messages: Option<crate::models::AdditionalResource>,
    /// Realtime bandwidth additional resources
    #[serde(rename = "realtimeBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_bandwidth: Option<crate::models::AdditionalResource>,
    /// Storage additional resources
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<crate::models::AdditionalResource>,
    /// User additional resources
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<crate::models::AdditionalResource>,
    /// GBHour additional resources
    #[serde(rename = "GBHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_hours: Option<crate::models::AdditionalResource>,
    /// Image transformation additional resources
    #[serde(rename = "imageTransformations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_transformations: Option<crate::models::AdditionalResource>,
    /// Credits additional resources
    #[serde(rename = "credits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<crate::models::AdditionalResource>,
}

impl UsageBillingPlan {
    /// Set bandwidth
    pub fn set_bandwidth(mut self, bandwidth: crate::models::AdditionalResource) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Get bandwidth
    pub fn bandwidth(&self) -> Option<&crate::models::AdditionalResource> {
        self.bandwidth.as_ref()
    }

    /// Set executions
    pub fn set_executions(mut self, executions: crate::models::AdditionalResource) -> Self {
        self.executions = Some(executions);
        self
    }

    /// Get executions
    pub fn executions(&self) -> Option<&crate::models::AdditionalResource> {
        self.executions.as_ref()
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

    /// Set realtime
    pub fn set_realtime(mut self, realtime: crate::models::AdditionalResource) -> Self {
        self.realtime = Some(realtime);
        self
    }

    /// Get realtime
    pub fn realtime(&self) -> Option<&crate::models::AdditionalResource> {
        self.realtime.as_ref()
    }

    /// Set realtime_messages
    pub fn set_realtime_messages(
        mut self,
        realtime_messages: crate::models::AdditionalResource,
    ) -> Self {
        self.realtime_messages = Some(realtime_messages);
        self
    }

    /// Get realtime_messages
    pub fn realtime_messages(&self) -> Option<&crate::models::AdditionalResource> {
        self.realtime_messages.as_ref()
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

    /// Set storage
    pub fn set_storage(mut self, storage: crate::models::AdditionalResource) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Get storage
    pub fn storage(&self) -> Option<&crate::models::AdditionalResource> {
        self.storage.as_ref()
    }

    /// Set users
    pub fn set_users(mut self, users: crate::models::AdditionalResource) -> Self {
        self.users = Some(users);
        self
    }

    /// Get users
    pub fn users(&self) -> Option<&crate::models::AdditionalResource> {
        self.users.as_ref()
    }

    /// Set gb_hours
    pub fn set_gb_hours(mut self, gb_hours: crate::models::AdditionalResource) -> Self {
        self.gb_hours = Some(gb_hours);
        self
    }

    /// Get gb_hours
    pub fn gb_hours(&self) -> Option<&crate::models::AdditionalResource> {
        self.gb_hours.as_ref()
    }

    /// Set image_transformations
    pub fn set_image_transformations(
        mut self,
        image_transformations: crate::models::AdditionalResource,
    ) -> Self {
        self.image_transformations = Some(image_transformations);
        self
    }

    /// Get image_transformations
    pub fn image_transformations(&self) -> Option<&crate::models::AdditionalResource> {
        self.image_transformations.as_ref()
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
