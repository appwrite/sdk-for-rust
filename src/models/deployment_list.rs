//! DeploymentList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Deployments List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DeploymentList {
    /// Total number of deployments that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of deployments.
    #[serde(rename = "deployments")]
    pub deployments: Vec<crate::models::Deployment>,
}

impl DeploymentList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get deployments
    pub fn deployments(&self) -> &Vec<crate::models::Deployment> {
        &self.deployments
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_list_creation() {
        let _model = <DeploymentList as Default>::default();
        let _ = _model.total();
        let _ = _model.deployments();
    }

    #[test]
    fn test_deployment_list_serialization() {
        let model = <DeploymentList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DeploymentList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
