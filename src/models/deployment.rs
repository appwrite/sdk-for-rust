//! Deployment model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Deployment {
    /// Deployment ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Deployment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Deployment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Type of deployment.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Resource ID.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// The entrypoint file to use to execute the deployment code.
    #[serde(rename = "entrypoint")]
    pub entrypoint: String,
    /// The code size in bytes.
    #[serde(rename = "sourceSize")]
    pub source_size: i64,
    /// The build output size in bytes.
    #[serde(rename = "buildSize")]
    pub build_size: i64,
    /// The total size in bytes (source and build output).
    #[serde(rename = "totalSize")]
    pub total_size: i64,
    /// The current build ID.
    #[serde(rename = "buildId")]
    pub build_id: String,
    /// Whether the deployment should be automatically activated.
    #[serde(rename = "activate")]
    pub activate: bool,
    /// Screenshot with light theme preference file ID.
    #[serde(rename = "screenshotLight")]
    pub screenshot_light: String,
    /// Screenshot with dark theme preference file ID.
    #[serde(rename = "screenshotDark")]
    pub screenshot_dark: String,
    /// The deployment status. Possible values are "waiting", "processing",
    /// "building", "ready", "canceled" and "failed".
    #[serde(rename = "status")]
    pub status: crate::enums::DeploymentStatus,
    /// The build logs.
    #[serde(rename = "buildLogs")]
    pub build_logs: String,
    /// The current build time in seconds.
    #[serde(rename = "buildDuration")]
    pub build_duration: i64,
    /// The name of the vcs provider repository
    #[serde(rename = "providerRepositoryName")]
    pub provider_repository_name: String,
    /// The name of the vcs provider repository owner
    #[serde(rename = "providerRepositoryOwner")]
    pub provider_repository_owner: String,
    /// The url of the vcs provider repository
    #[serde(rename = "providerRepositoryUrl")]
    pub provider_repository_url: String,
    /// The commit hash of the vcs commit
    #[serde(rename = "providerCommitHash")]
    pub provider_commit_hash: String,
    /// The url of vcs commit author
    #[serde(rename = "providerCommitAuthorUrl")]
    pub provider_commit_author_url: String,
    /// The name of vcs commit author
    #[serde(rename = "providerCommitAuthor")]
    pub provider_commit_author: String,
    /// The commit message
    #[serde(rename = "providerCommitMessage")]
    pub provider_commit_message: String,
    /// The url of the vcs commit
    #[serde(rename = "providerCommitUrl")]
    pub provider_commit_url: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranch")]
    pub provider_branch: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranchUrl")]
    pub provider_branch_url: String,
}

impl Deployment {
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

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get entrypoint
    pub fn entrypoint(&self) -> &String {
        &self.entrypoint
    }

    /// Get source_size
    pub fn source_size(&self) -> &i64 {
        &self.source_size
    }

    /// Get build_size
    pub fn build_size(&self) -> &i64 {
        &self.build_size
    }

    /// Get total_size
    pub fn total_size(&self) -> &i64 {
        &self.total_size
    }

    /// Get build_id
    pub fn build_id(&self) -> &String {
        &self.build_id
    }

    /// Get activate
    pub fn activate(&self) -> &bool {
        &self.activate
    }

    /// Get screenshot_light
    pub fn screenshot_light(&self) -> &String {
        &self.screenshot_light
    }

    /// Get screenshot_dark
    pub fn screenshot_dark(&self) -> &String {
        &self.screenshot_dark
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::DeploymentStatus {
        &self.status
    }

    /// Get build_logs
    pub fn build_logs(&self) -> &String {
        &self.build_logs
    }

    /// Get build_duration
    pub fn build_duration(&self) -> &i64 {
        &self.build_duration
    }

    /// Get provider_repository_name
    pub fn provider_repository_name(&self) -> &String {
        &self.provider_repository_name
    }

    /// Get provider_repository_owner
    pub fn provider_repository_owner(&self) -> &String {
        &self.provider_repository_owner
    }

    /// Get provider_repository_url
    pub fn provider_repository_url(&self) -> &String {
        &self.provider_repository_url
    }

    /// Get provider_commit_hash
    pub fn provider_commit_hash(&self) -> &String {
        &self.provider_commit_hash
    }

    /// Get provider_commit_author_url
    pub fn provider_commit_author_url(&self) -> &String {
        &self.provider_commit_author_url
    }

    /// Get provider_commit_author
    pub fn provider_commit_author(&self) -> &String {
        &self.provider_commit_author
    }

    /// Get provider_commit_message
    pub fn provider_commit_message(&self) -> &String {
        &self.provider_commit_message
    }

    /// Get provider_commit_url
    pub fn provider_commit_url(&self) -> &String {
        &self.provider_commit_url
    }

    /// Get provider_branch
    pub fn provider_branch(&self) -> &String {
        &self.provider_branch
    }

    /// Get provider_branch_url
    pub fn provider_branch_url(&self) -> &String {
        &self.provider_branch_url
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_creation() {
        let _model = <Deployment as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.r#type();
        let _ = _model.resource_id();
        let _ = _model.resource_type();
        let _ = _model.entrypoint();
        let _ = _model.source_size();
        let _ = _model.build_size();
        let _ = _model.total_size();
        let _ = _model.build_id();
        let _ = _model.activate();
        let _ = _model.screenshot_light();
        let _ = _model.screenshot_dark();
        let _ = _model.status();
        let _ = _model.build_logs();
        let _ = _model.build_duration();
        let _ = _model.provider_repository_name();
        let _ = _model.provider_repository_owner();
        let _ = _model.provider_repository_url();
        let _ = _model.provider_commit_hash();
        let _ = _model.provider_commit_author_url();
        let _ = _model.provider_commit_author();
        let _ = _model.provider_commit_message();
        let _ = _model.provider_commit_url();
        let _ = _model.provider_branch();
        let _ = _model.provider_branch_url();
    }

    #[test]
    fn test_deployment_serialization() {
        let model = <Deployment as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Deployment, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
