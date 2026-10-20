//! Site model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Site
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Site {
    /// Site ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Site creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Site update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Site name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Optional description for the site.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Site enabled.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Is the site deployed with the latest configuration? This is set to false if
    /// you've changed an environment variables, entrypoint, commands, or other
    /// settings that needs redeploy to be applied. When the value is false,
    /// redeploy the site to update it with the latest configuration.
    #[serde(rename = "live")]
    #[serde(default = "default_live")]
    pub live: bool,
    /// When disabled, request logs will exclude logs and errors, and site
    /// responses will be slightly faster.
    #[serde(rename = "logging")]
    #[serde(default = "default_logging")]
    pub logging: bool,
    /// Site framework.
    #[serde(rename = "framework")]
    #[serde(default = "default_framework")]
    pub framework: String,
    /// How many days to keep the non-active deployments before they will be
    /// automatically deleted.
    #[serde(rename = "deploymentRetention")]
    #[serde(default = "default_deployment_retention")]
    pub deployment_retention: i64,
    /// Site's active deployment ID.
    #[serde(rename = "deploymentId")]
    #[serde(default = "default_deployment_id")]
    pub deployment_id: String,
    /// Active deployment creation date in ISO 8601 format.
    #[serde(rename = "deploymentCreatedAt")]
    #[serde(default = "default_deployment_created_at")]
    pub deployment_created_at: String,
    /// Screenshot of active deployment with light theme preference file ID.
    #[serde(rename = "deploymentScreenshotLight")]
    #[serde(default = "default_deployment_screenshot_light")]
    pub deployment_screenshot_light: String,
    /// Screenshot of active deployment with dark theme preference file ID.
    #[serde(rename = "deploymentScreenshotDark")]
    #[serde(default = "default_deployment_screenshot_dark")]
    pub deployment_screenshot_dark: String,
    /// Site's latest deployment ID.
    #[serde(rename = "latestDeploymentId")]
    #[serde(default = "default_latest_deployment_id")]
    pub latest_deployment_id: String,
    /// Latest deployment creation date in ISO 8601 format.
    #[serde(rename = "latestDeploymentCreatedAt")]
    #[serde(default = "default_latest_deployment_created_at")]
    pub latest_deployment_created_at: String,
    /// Status of latest deployment. Possible values are "waiting", "processing",
    /// "building", "ready", and "failed".
    #[serde(rename = "latestDeploymentStatus")]
    #[serde(default = "default_latest_deployment_status")]
    pub latest_deployment_status: String,
    /// Allowed permission scopes.
    #[serde(rename = "scopes")]
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    /// Site variables.
    #[serde(rename = "vars")]
    #[serde(default = "default_vars")]
    pub vars: Vec<crate::models::Variable>,
    /// Site request timeout in seconds.
    #[serde(rename = "timeout")]
    #[serde(default = "default_timeout")]
    pub timeout: i64,
    /// The install command used to install the site dependencies.
    #[serde(rename = "installCommand")]
    #[serde(default = "default_install_command")]
    pub install_command: String,
    /// The build command used to build the site.
    #[serde(rename = "buildCommand")]
    #[serde(default = "default_build_command")]
    pub build_command: String,
    /// Custom command to use when starting site runtime.
    #[serde(rename = "startCommand")]
    #[serde(default = "default_start_command")]
    pub start_command: String,
    /// The directory where the site build output is located.
    #[serde(rename = "outputDirectory")]
    #[serde(default = "default_output_directory")]
    pub output_directory: String,
    /// Site VCS (Version Control System) installation id.
    #[serde(rename = "installationId")]
    #[serde(default = "default_installation_id")]
    pub installation_id: String,
    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId")]
    #[serde(default = "default_provider_repository_id")]
    pub provider_repository_id: String,
    /// VCS (Version Control System) branch name
    #[serde(rename = "providerBranch")]
    #[serde(default = "default_provider_branch")]
    pub provider_branch: String,
    /// Path to site in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory")]
    #[serde(default = "default_provider_root_directory")]
    pub provider_root_directory: String,
    /// Is VCS (Version Control System) connection is in silent mode? When in
    /// silence mode, no comments will be posted on the repository pull or merge
    /// requests
    #[serde(rename = "providerSilentMode")]
    #[serde(default = "default_provider_silent_mode")]
    pub provider_silent_mode: bool,
    /// List of branch name patterns that trigger automatic deployments. Supports
    /// glob wildcards. Empty list deploys on all branches.
    #[serde(rename = "providerBranches")]
    #[serde(default = "default_provider_branches")]
    pub provider_branches: Vec<String>,
    /// List of file path patterns that trigger automatic deployments. Supports
    /// glob wildcards. Empty list deploys on all file changes.
    #[serde(rename = "providerPaths")]
    #[serde(default = "default_provider_paths")]
    pub provider_paths: Vec<String>,
    /// Machine specification for deployment builds.
    #[serde(rename = "buildSpecification")]
    #[serde(default = "default_build_specification")]
    pub build_specification: String,
    /// Machine specification for SSR executions.
    #[serde(rename = "runtimeSpecification")]
    #[serde(default = "default_runtime_specification")]
    pub runtime_specification: String,
    /// Site build runtime.
    #[serde(rename = "buildRuntime")]
    #[serde(default = "default_build_runtime")]
    pub build_runtime: String,
    /// Site framework adapter.
    #[serde(rename = "adapter")]
    pub adapter: String,
    /// Name of fallback file to use instead of 404 page. If null, Appwrite 404
    /// page will be displayed.
    #[serde(rename = "fallbackFile")]
    pub fallback_file: String,
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

fn default_name() -> String {
    "".to_string()
}

fn default_description() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    true
}

fn default_live() -> bool {
    true
}

fn default_logging() -> bool {
    true
}

fn default_framework() -> String {
    "".to_string()
}

fn default_deployment_retention() -> i64 {
    0
}

fn default_deployment_id() -> String {
    "".to_string()
}

fn default_deployment_created_at() -> String {
    "".to_string()
}

fn default_deployment_screenshot_light() -> String {
    "".to_string()
}

fn default_deployment_screenshot_dark() -> String {
    "".to_string()
}

fn default_latest_deployment_id() -> String {
    "".to_string()
}

fn default_latest_deployment_created_at() -> String {
    "".to_string()
}

fn default_latest_deployment_status() -> String {
    "".to_string()
}

fn default_scopes() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_vars() -> Vec<crate::models::Variable> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_timeout() -> i64 {
    15
}

fn default_install_command() -> String {
    "".to_string()
}

fn default_build_command() -> String {
    "".to_string()
}

fn default_start_command() -> String {
    "".to_string()
}

fn default_output_directory() -> String {
    "".to_string()
}

fn default_installation_id() -> String {
    "".to_string()
}

fn default_provider_repository_id() -> String {
    "".to_string()
}

fn default_provider_branch() -> String {
    "".to_string()
}

fn default_provider_root_directory() -> String {
    "".to_string()
}

fn default_provider_silent_mode() -> bool {
    false
}

fn default_provider_branches() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_provider_paths() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_build_specification() -> String {
    "s-1vcpu-512mb".to_string()
}

fn default_runtime_specification() -> String {
    "s-1vcpu-512mb".to_string()
}

fn default_build_runtime() -> String {
    "".to_string()
}

impl Site {
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

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get live
    pub fn live(&self) -> &bool {
        &self.live
    }

    /// Get logging
    pub fn logging(&self) -> &bool {
        &self.logging
    }

    /// Get framework
    pub fn framework(&self) -> &String {
        &self.framework
    }

    /// Get deployment_retention
    pub fn deployment_retention(&self) -> &i64 {
        &self.deployment_retention
    }

    /// Get deployment_id
    pub fn deployment_id(&self) -> &String {
        &self.deployment_id
    }

    /// Get deployment_created_at
    pub fn deployment_created_at(&self) -> &String {
        &self.deployment_created_at
    }

    /// Get deployment_screenshot_light
    pub fn deployment_screenshot_light(&self) -> &String {
        &self.deployment_screenshot_light
    }

    /// Get deployment_screenshot_dark
    pub fn deployment_screenshot_dark(&self) -> &String {
        &self.deployment_screenshot_dark
    }

    /// Get latest_deployment_id
    pub fn latest_deployment_id(&self) -> &String {
        &self.latest_deployment_id
    }

    /// Get latest_deployment_created_at
    pub fn latest_deployment_created_at(&self) -> &String {
        &self.latest_deployment_created_at
    }

    /// Get latest_deployment_status
    pub fn latest_deployment_status(&self) -> &String {
        &self.latest_deployment_status
    }

    /// Get scopes
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    /// Get vars
    pub fn vars(&self) -> &Vec<crate::models::Variable> {
        &self.vars
    }

    /// Get timeout
    pub fn timeout(&self) -> &i64 {
        &self.timeout
    }

    /// Get install_command
    pub fn install_command(&self) -> &String {
        &self.install_command
    }

    /// Get build_command
    pub fn build_command(&self) -> &String {
        &self.build_command
    }

    /// Get start_command
    pub fn start_command(&self) -> &String {
        &self.start_command
    }

    /// Get output_directory
    pub fn output_directory(&self) -> &String {
        &self.output_directory
    }

    /// Get installation_id
    pub fn installation_id(&self) -> &String {
        &self.installation_id
    }

    /// Get provider_repository_id
    pub fn provider_repository_id(&self) -> &String {
        &self.provider_repository_id
    }

    /// Get provider_branch
    pub fn provider_branch(&self) -> &String {
        &self.provider_branch
    }

    /// Get provider_root_directory
    pub fn provider_root_directory(&self) -> &String {
        &self.provider_root_directory
    }

    /// Get provider_silent_mode
    pub fn provider_silent_mode(&self) -> &bool {
        &self.provider_silent_mode
    }

    /// Get provider_branches
    pub fn provider_branches(&self) -> &Vec<String> {
        &self.provider_branches
    }

    /// Get provider_paths
    pub fn provider_paths(&self) -> &Vec<String> {
        &self.provider_paths
    }

    /// Get build_specification
    pub fn build_specification(&self) -> &String {
        &self.build_specification
    }

    /// Get runtime_specification
    pub fn runtime_specification(&self) -> &String {
        &self.runtime_specification
    }

    /// Get build_runtime
    pub fn build_runtime(&self) -> &String {
        &self.build_runtime
    }

    /// Get adapter
    pub fn adapter(&self) -> &String {
        &self.adapter
    }

    /// Get fallback_file
    pub fn fallback_file(&self) -> &String {
        &self.fallback_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_creation() {
        let _model = <Site as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.description();
        let _ = _model.enabled();
        let _ = _model.live();
        let _ = _model.logging();
        let _ = _model.framework();
        let _ = _model.deployment_retention();
        let _ = _model.deployment_id();
        let _ = _model.deployment_created_at();
        let _ = _model.deployment_screenshot_light();
        let _ = _model.deployment_screenshot_dark();
        let _ = _model.latest_deployment_id();
        let _ = _model.latest_deployment_created_at();
        let _ = _model.latest_deployment_status();
        let _ = _model.scopes();
        let _ = _model.vars();
        let _ = _model.timeout();
        let _ = _model.install_command();
        let _ = _model.build_command();
        let _ = _model.start_command();
        let _ = _model.output_directory();
        let _ = _model.installation_id();
        let _ = _model.provider_repository_id();
        let _ = _model.provider_branch();
        let _ = _model.provider_root_directory();
        let _ = _model.provider_silent_mode();
        let _ = _model.provider_branches();
        let _ = _model.provider_paths();
        let _ = _model.build_specification();
        let _ = _model.runtime_specification();
        let _ = _model.build_runtime();
        let _ = _model.adapter();
        let _ = _model.fallback_file();
    }

    #[test]
    fn test_site_serialization() {
        let model = <Site as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Site, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
