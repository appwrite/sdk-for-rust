//! Block model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Block {
    /// Block creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Resource type that is blocked
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Resource identifier that is blocked
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Reason for the block. Can be null if no reason was provided.
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Block expiration date in ISO 8601 format. Can be null if the block does not
    /// expire.
    #[serde(rename = "expiredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Name of the project this block applies to.
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// Region of the project this block applies to.
    #[serde(rename = "region")]
    pub region: String,
    /// Name of the organization that owns the project.
    #[serde(rename = "organizationName")]
    pub organization_name: String,
    /// ID of the organization that owns the project.
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    /// Billing plan of the organization that owns the project.
    #[serde(rename = "billingPlan")]
    pub billing_plan: String,
}

impl Block {
    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Set reason
    pub fn set_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Get reason
    pub fn reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Set expired_at
    pub fn set_expired_at(mut self, expired_at: String) -> Self {
        self.expired_at = Some(expired_at);
        self
    }

    /// Get expired_at
    pub fn expired_at(&self) -> Option<&String> {
        self.expired_at.as_ref()
    }

    /// Get project_name
    pub fn project_name(&self) -> &String {
        &self.project_name
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Get organization_name
    pub fn organization_name(&self) -> &String {
        &self.organization_name
    }

    /// Get organization_id
    pub fn organization_id(&self) -> &String {
        &self.organization_id
    }

    /// Get billing_plan
    pub fn billing_plan(&self) -> &String {
        &self.billing_plan
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let _model = <Block as Default>::default();
        let _ = _model.created_at();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.project_name();
        let _ = _model.region();
        let _ = _model.organization_name();
        let _ = _model.organization_id();
        let _ = _model.billing_plan();
    }

    #[test]
    fn test_block_serialization() {
        let model = <Block as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Block, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
