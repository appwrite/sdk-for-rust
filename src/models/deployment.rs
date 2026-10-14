//! Deployment model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Deployment {
    /// Deployment ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Deployment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Deployment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Type of deployment.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Resource ID.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Resource type.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    /// The entrypoint file to use to execute the deployment code.
    #[serde(rename = "entrypoint")]
    #[serde(default = "default_entrypoint")]
    pub entrypoint: String,
    /// The code size in bytes.
    #[serde(rename = "sourceSize")]
    #[serde(default = "default_source_size")]
    pub source_size: i64,
    /// The build output size in bytes.
    #[serde(rename = "buildSize")]
    #[serde(default = "default_build_size")]
    pub build_size: i64,
    /// The total size in bytes (source and build output).
    #[serde(rename = "totalSize")]
    #[serde(default = "default_total_size")]
    pub total_size: i64,
    /// The current build ID.
    #[serde(rename = "buildId")]
    #[serde(default = "default_build_id")]
    pub build_id: String,
    /// Whether the deployment should be automatically activated.
    #[serde(rename = "activate")]
    #[serde(default = "default_activate")]
    pub activate: bool,
    /// Screenshot with light theme preference file ID.
    #[serde(rename = "screenshotLight")]
    #[serde(default = "default_screenshot_light")]
    pub screenshot_light: String,
    /// Screenshot with dark theme preference file ID.
    #[serde(rename = "screenshotDark")]
    #[serde(default = "default_screenshot_dark")]
    pub screenshot_dark: String,
    /// The deployment status. Possible values are "waiting", "processing",
    /// "building", "ready", "canceled" and "failed".
    #[serde(rename = "status")]
    pub status: crate::enums::DeploymentStatus,
    /// The build logs.
    #[serde(rename = "buildLogs")]
    #[serde(default = "default_build_logs")]
    pub build_logs: String,
    /// The current build time in seconds.
    #[serde(rename = "buildDuration")]
    #[serde(default = "default_build_duration")]
    pub build_duration: i64,
    /// The name of the vcs provider repository
    #[serde(rename = "providerRepositoryName")]
    #[serde(default = "default_provider_repository_name")]
    pub provider_repository_name: String,
    /// The name of the vcs provider repository owner
    #[serde(rename = "providerRepositoryOwner")]
    #[serde(default = "default_provider_repository_owner")]
    pub provider_repository_owner: String,
    /// The url of the vcs provider repository
    #[serde(rename = "providerRepositoryUrl")]
    #[serde(default = "default_provider_repository_url")]
    pub provider_repository_url: String,
    /// The commit hash of the vcs commit
    #[serde(rename = "providerCommitHash")]
    #[serde(default = "default_provider_commit_hash")]
    pub provider_commit_hash: String,
    /// The url of vcs commit author
    #[serde(rename = "providerCommitAuthorUrl")]
    #[serde(default = "default_provider_commit_author_url")]
    pub provider_commit_author_url: String,
    /// The name of vcs commit author
    #[serde(rename = "providerCommitAuthor")]
    #[serde(default = "default_provider_commit_author")]
    pub provider_commit_author: String,
    /// The commit message
    #[serde(rename = "providerCommitMessage")]
    #[serde(default = "default_provider_commit_message")]
    pub provider_commit_message: String,
    /// The url of the vcs commit
    #[serde(rename = "providerCommitUrl")]
    #[serde(default = "default_provider_commit_url")]
    pub provider_commit_url: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranch")]
    #[serde(default = "default_provider_branch")]
    pub provider_branch: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranchUrl")]
    #[serde(default = "default_provider_branch_url")]
    pub provider_branch_url: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_type() -> String {
    "".to_string()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_resource_type() -> String {
    "".to_string()
}

fn default_entrypoint() -> String {
    "".to_string()
}

fn default_source_size() -> i64 {
    0
}

fn default_build_size() -> i64 {
    0
}

fn default_total_size() -> i64 {
    0
}

fn default_build_id() -> String {
    "".to_string()
}

fn default_activate() -> bool {
    false
}

fn default_screenshot_light() -> String {
    "".to_string()
}

fn default_screenshot_dark() -> String {
    "".to_string()
}

fn default_build_logs() -> String {
    "".to_string()
}

fn default_build_duration() -> i64 {
    0
}

fn default_provider_repository_name() -> String {
    "".to_string()
}

fn default_provider_repository_owner() -> String {
    "".to_string()
}

fn default_provider_repository_url() -> String {
    "".to_string()
}

fn default_provider_commit_hash() -> String {
    "".to_string()
}

fn default_provider_commit_author_url() -> String {
    "".to_string()
}

fn default_provider_commit_author() -> String {
    "".to_string()
}

fn default_provider_commit_message() -> String {
    "".to_string()
}

fn default_provider_commit_url() -> String {
    "".to_string()
}

fn default_provider_branch() -> String {
    "".to_string()
}

fn default_provider_branch_url() -> String {
    "".to_string()
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
