//! Function model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Function
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Function {
    /// Function ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Function creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Function update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Execution permissions.
    #[serde(rename = "execute")]
    pub execute: Vec<String>,
    /// Function name.
    #[serde(rename = "name")]
    pub name: String,
    /// Function enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Is the function deployed with the latest configuration? This is set to
    /// false if you've changed an environment variables, entrypoint, commands, or
    /// other settings that needs redeploy to be applied. When the value is false,
    /// redeploy the function to update it with the latest configuration.
    #[serde(rename = "live")]
    pub live: bool,
    /// When disabled, executions will exclude logs and errors, and will be
    /// slightly faster.
    #[serde(rename = "logging")]
    pub logging: bool,
    /// Function execution and build runtime.
    #[serde(rename = "runtime")]
    pub runtime: String,
    /// How many days to keep the non-active deployments before they will be
    /// automatically deleted.
    #[serde(rename = "deploymentRetention")]
    pub deployment_retention: i64,
    /// Function's active deployment ID.
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// Active deployment creation date in ISO 8601 format.
    #[serde(rename = "deploymentCreatedAt")]
    pub deployment_created_at: String,
    /// Function's latest deployment ID.
    #[serde(rename = "latestDeploymentId")]
    pub latest_deployment_id: String,
    /// Latest deployment creation date in ISO 8601 format.
    #[serde(rename = "latestDeploymentCreatedAt")]
    pub latest_deployment_created_at: String,
    /// Status of latest deployment. Possible values are "waiting", "processing",
    /// "building", "ready", and "failed".
    #[serde(rename = "latestDeploymentStatus")]
    pub latest_deployment_status: String,
    /// Allowed permission scopes.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    /// Function variables.
    #[serde(rename = "vars")]
    pub vars: Vec<crate::models::Variable>,
    /// Function trigger events.
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// Function execution schedule in CRON format.
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// Function execution timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: i64,
    /// The entrypoint file used to execute the deployment.
    #[serde(rename = "entrypoint")]
    pub entrypoint: String,
    /// The build command used to build the deployment.
    #[serde(rename = "commands")]
    pub commands: String,
    /// Version of Open Runtimes used for the function.
    #[serde(rename = "version")]
    pub version: String,
    /// Function VCS (Version Control System) installation id.
    #[serde(rename = "installationId")]
    pub installation_id: String,
    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId")]
    pub provider_repository_id: String,
    /// VCS (Version Control System) branch name
    #[serde(rename = "providerBranch")]
    pub provider_branch: String,
    /// Path to function in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory")]
    pub provider_root_directory: String,
    /// Is VCS (Version Control System) connection is in silent mode? When in
    /// silence mode, no comments will be posted on the repository pull or merge
    /// requests
    #[serde(rename = "providerSilentMode")]
    pub provider_silent_mode: bool,
    /// Machine specification for deployment builds.
    #[serde(rename = "buildSpecification")]
    pub build_specification: String,
    /// Machine specification for executions.
    #[serde(rename = "runtimeSpecification")]
    pub runtime_specification: String,
}

impl Function {
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

    /// Get execute
    pub fn execute(&self) -> &Vec<String> {
        &self.execute
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
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

    /// Get runtime
    pub fn runtime(&self) -> &String {
        &self.runtime
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

    /// Get events
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }

    /// Get schedule
    pub fn schedule(&self) -> &String {
        &self.schedule
    }

    /// Get timeout
    pub fn timeout(&self) -> &i64 {
        &self.timeout
    }

    /// Get entrypoint
    pub fn entrypoint(&self) -> &String {
        &self.entrypoint
    }

    /// Get commands
    pub fn commands(&self) -> &String {
        &self.commands
    }

    /// Get version
    pub fn version(&self) -> &String {
        &self.version
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

    /// Get build_specification
    pub fn build_specification(&self) -> &String {
        &self.build_specification
    }

    /// Get runtime_specification
    pub fn runtime_specification(&self) -> &String {
        &self.runtime_specification
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_creation() {
        let _model = <Function as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.execute();
        let _ = _model.name();
        let _ = _model.enabled();
        let _ = _model.live();
        let _ = _model.logging();
        let _ = _model.runtime();
        let _ = _model.deployment_retention();
        let _ = _model.deployment_id();
        let _ = _model.deployment_created_at();
        let _ = _model.latest_deployment_id();
        let _ = _model.latest_deployment_created_at();
        let _ = _model.latest_deployment_status();
        let _ = _model.scopes();
        let _ = _model.vars();
        let _ = _model.events();
        let _ = _model.schedule();
        let _ = _model.timeout();
        let _ = _model.entrypoint();
        let _ = _model.commands();
        let _ = _model.version();
        let _ = _model.installation_id();
        let _ = _model.provider_repository_id();
        let _ = _model.provider_branch();
        let _ = _model.provider_root_directory();
        let _ = _model.provider_silent_mode();
        let _ = _model.build_specification();
        let _ = _model.runtime_specification();
    }

    #[test]
    fn test_function_serialization() {
        let model = <Function as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Function, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
