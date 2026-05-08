//! ProxyRule model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProxyRule {
    /// Rule ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Rule creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Rule update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Domain name.
    #[serde(rename = "domain")]
    pub domain: String,
    /// Action definition for the rule. Possible values are "api", "deployment", or
    /// "redirect"
    #[serde(rename = "type")]
    pub r#type: String,
    /// Defines how the rule was created. Possible values are "manual" or
    /// "deployment"
    #[serde(rename = "trigger")]
    pub trigger: String,
    /// URL to redirect to. Used if type is "redirect"
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    /// Status code to apply during redirect. Used if type is "redirect"
    #[serde(rename = "redirectStatusCode")]
    pub redirect_status_code: i64,
    /// ID of deployment. Used if type is "deployment"
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// Type of deployment. Possible values are "function", "site". Used if rule's
    /// type is "deployment".
    #[serde(rename = "deploymentResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_resource_type: Option<crate::enums::ProxyRuleDeploymentResourceType>,
    /// ID of deployment's resource (site or function ID). Used if type is
    /// "deployment"
    #[serde(rename = "deploymentResourceId")]
    pub deployment_resource_id: String,
    /// Name of Git branch that updates rule. Used if type is "deployment"
    #[serde(rename = "deploymentVcsProviderBranch")]
    pub deployment_vcs_provider_branch: String,
    /// Domain verification status. Possible values are "unverified", "verifying",
    /// "verified"
    #[serde(rename = "status")]
    pub status: crate::enums::ProxyRuleStatus,
    /// Logs from rule verification or certificate generation. Certificate
    /// generation logs are prioritized if both are available.
    #[serde(rename = "logs")]
    pub logs: String,
    /// Certificate auto-renewal date in ISO 8601 format.
    #[serde(rename = "renewAt")]
    pub renew_at: String,
}

impl ProxyRule {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get trigger
    pub fn trigger(&self) -> &String {
        &self.trigger
    }

    /// Get redirect_url
    pub fn redirect_url(&self) -> &String {
        &self.redirect_url
    }

    /// Get redirect_status_code
    pub fn redirect_status_code(&self) -> &i64 {
        &self.redirect_status_code
    }

    /// Get deployment_id
    pub fn deployment_id(&self) -> &String {
        &self.deployment_id
    }

    /// Set deployment_resource_type
    pub fn set_deployment_resource_type(mut self, deployment_resource_type: crate::enums::ProxyRuleDeploymentResourceType) -> Self {
        self.deployment_resource_type = Some(deployment_resource_type);
        self
    }

    /// Get deployment_resource_type
    pub fn deployment_resource_type(&self) -> Option<&crate::enums::ProxyRuleDeploymentResourceType> {
        self.deployment_resource_type.as_ref()
    }

    /// Get deployment_resource_id
    pub fn deployment_resource_id(&self) -> &String {
        &self.deployment_resource_id
    }

    /// Get deployment_vcs_provider_branch
    pub fn deployment_vcs_provider_branch(&self) -> &String {
        &self.deployment_vcs_provider_branch
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::ProxyRuleStatus {
        &self.status
    }

    /// Get logs
    pub fn logs(&self) -> &String {
        &self.logs
    }

    /// Get renew_at
    pub fn renew_at(&self) -> &String {
        &self.renew_at
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_rule_creation() {
        let _model = <ProxyRule as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.domain();
        let _ = _model.r#type();
        let _ = _model.trigger();
        let _ = _model.redirect_url();
        let _ = _model.redirect_status_code();
        let _ = _model.deployment_id();
        let _ = _model.deployment_resource_id();
        let _ = _model.deployment_vcs_provider_branch();
        let _ = _model.status();
        let _ = _model.logs();
        let _ = _model.renew_at();
    }

    #[test]
    fn test_proxy_rule_serialization() {
        let model = <ProxyRule as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProxyRule, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
