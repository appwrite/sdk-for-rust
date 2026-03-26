//! Sites service for Appwrite SDK

use crate::client::Client;
use crate::input_file::InputFile;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Sites Service allows you view, create and manage your web applications.
#[derive(Debug, Clone)]
pub struct Sites {
    client: Client,
}

impl Sites {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the project's sites. You can use the query params to
    /// filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::SiteList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/sites".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new site.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        site_id: impl Into<String>,
        name: impl Into<String>,
        framework: crate::enums::Framework,
        build_runtime: crate::enums::BuildRuntime,
        enabled: Option<bool>,
        logging: Option<bool>,
        timeout: Option<i64>,
        install_command: Option<&str>,
        build_command: Option<&str>,
        start_command: Option<&str>,
        output_directory: Option<&str>,
        adapter: Option<crate::enums::Adapter>,
        installation_id: Option<&str>,
        fallback_file: Option<&str>,
        provider_repository_id: Option<&str>,
        provider_branch: Option<&str>,
        provider_silent_mode: Option<bool>,
        provider_root_directory: Option<&str>,
        build_specification: Option<&str>,
        runtime_specification: Option<&str>,
        deployment_retention: Option<i64>,
    ) -> crate::error::Result<crate::models::Site> {
        let mut params = HashMap::new();
        params.insert("siteId".to_string(), json!(site_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("framework".to_string(), json!(framework));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = logging {
            params.insert("logging".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = install_command {
            params.insert("installCommand".to_string(), json!(value));
        }
        if let Some(value) = build_command {
            params.insert("buildCommand".to_string(), json!(value));
        }
        if let Some(value) = start_command {
            params.insert("startCommand".to_string(), json!(value));
        }
        if let Some(value) = output_directory {
            params.insert("outputDirectory".to_string(), json!(value));
        }
        params.insert("buildRuntime".to_string(), json!(build_runtime));
        if let Some(value) = adapter {
            params.insert("adapter".to_string(), json!(value));
        }
        if let Some(value) = installation_id {
            params.insert("installationId".to_string(), json!(value));
        }
        if let Some(value) = fallback_file {
            params.insert("fallbackFile".to_string(), json!(value));
        }
        if let Some(value) = provider_repository_id {
            params.insert("providerRepositoryId".to_string(), json!(value));
        }
        if let Some(value) = provider_branch {
            params.insert("providerBranch".to_string(), json!(value));
        }
        if let Some(value) = provider_silent_mode {
            params.insert("providerSilentMode".to_string(), json!(value));
        }
        if let Some(value) = provider_root_directory {
            params.insert("providerRootDirectory".to_string(), json!(value));
        }
        if let Some(value) = build_specification {
            params.insert("buildSpecification".to_string(), json!(value));
        }
        if let Some(value) = runtime_specification {
            params.insert("runtimeSpecification".to_string(), json!(value));
        }
        if let Some(value) = deployment_retention {
            params.insert("deploymentRetention".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all frameworks that are currently available on the server
    /// instance.
    pub async fn list_frameworks(
        &self,
    ) -> crate::error::Result<crate::models::FrameworkList> {
        let params = HashMap::new();

        let path = "/sites/frameworks".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List allowed site specifications for this instance.
    pub async fn list_specifications(
        &self,
    ) -> crate::error::Result<crate::models::SpecificationList> {
        let params = HashMap::new();

        let path = "/sites/specifications".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a site by its unique ID.
    pub async fn get(
        &self,
        site_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Site> {
        let params = HashMap::new();

        let path = "/sites/{siteId}".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update site by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        site_id: impl Into<String>,
        name: impl Into<String>,
        framework: crate::enums::Framework,
        enabled: Option<bool>,
        logging: Option<bool>,
        timeout: Option<i64>,
        install_command: Option<&str>,
        build_command: Option<&str>,
        start_command: Option<&str>,
        output_directory: Option<&str>,
        build_runtime: Option<crate::enums::BuildRuntime>,
        adapter: Option<crate::enums::Adapter>,
        fallback_file: Option<&str>,
        installation_id: Option<&str>,
        provider_repository_id: Option<&str>,
        provider_branch: Option<&str>,
        provider_silent_mode: Option<bool>,
        provider_root_directory: Option<&str>,
        build_specification: Option<&str>,
        runtime_specification: Option<&str>,
        deployment_retention: Option<i64>,
    ) -> crate::error::Result<crate::models::Site> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("framework".to_string(), json!(framework));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = logging {
            params.insert("logging".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = install_command {
            params.insert("installCommand".to_string(), json!(value));
        }
        if let Some(value) = build_command {
            params.insert("buildCommand".to_string(), json!(value));
        }
        if let Some(value) = start_command {
            params.insert("startCommand".to_string(), json!(value));
        }
        if let Some(value) = output_directory {
            params.insert("outputDirectory".to_string(), json!(value));
        }
        if let Some(value) = build_runtime {
            params.insert("buildRuntime".to_string(), json!(value));
        }
        if let Some(value) = adapter {
            params.insert("adapter".to_string(), json!(value));
        }
        if let Some(value) = fallback_file {
            params.insert("fallbackFile".to_string(), json!(value));
        }
        if let Some(value) = installation_id {
            params.insert("installationId".to_string(), json!(value));
        }
        if let Some(value) = provider_repository_id {
            params.insert("providerRepositoryId".to_string(), json!(value));
        }
        if let Some(value) = provider_branch {
            params.insert("providerBranch".to_string(), json!(value));
        }
        if let Some(value) = provider_silent_mode {
            params.insert("providerSilentMode".to_string(), json!(value));
        }
        if let Some(value) = provider_root_directory {
            params.insert("providerRootDirectory".to_string(), json!(value));
        }
        if let Some(value) = build_specification {
            params.insert("buildSpecification".to_string(), json!(value));
        }
        if let Some(value) = runtime_specification {
            params.insert("runtimeSpecification".to_string(), json!(value));
        }
        if let Some(value) = deployment_retention {
            params.insert("deploymentRetention".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a site by its unique ID.
    pub async fn delete(
        &self,
        site_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the site active deployment. Use this endpoint to switch the code
    /// deployment that should be used when visitor opens your site.
    pub async fn update_site_deployment(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Site> {
        let mut params = HashMap::new();
        params.insert("deploymentId".to_string(), json!(deployment_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployment".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the site's code deployments. You can use the query params
    /// to filter your results.
    pub async fn list_deployments(
        &self,
        site_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::DeploymentList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/sites/{siteId}/deployments".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new site code deployment. Use this endpoint to upload a new
    /// version of your site code. To activate your newly uploaded code, you'll
    /// need to update the site's deployment to use your new deployment ID.
    pub async fn create_deployment(
        &self,
        site_id: impl Into<String>,
        code: InputFile,
        install_command: Option<&str>,
        build_command: Option<&str>,
        output_directory: Option<&str>,
        activate: Option<bool>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        if let Some(value) = install_command {
            params.insert("installCommand".to_string(), json!(value));
        }
        if let Some(value) = build_command {
            params.insert("buildCommand".to_string(), json!(value));
        }
        if let Some(value) = output_directory {
            params.insert("outputDirectory".to_string(), json!(value));
        }
        if let Some(value) = activate {
            params.insert("activate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let path = "/sites/{siteId}/deployments".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.file_upload(&path, Some(api_headers), params, "code", code, None).await
    }

    /// Create a new build for an existing site deployment. This endpoint allows
    /// you to rebuild a deployment with the updated site configuration, including
    /// its commands and output directory if they have been modified. The build
    /// process will be queued and executed asynchronously. The original
    /// deployment's code will be preserved and used for the new build.
    pub async fn create_duplicate_deployment(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        params.insert("deploymentId".to_string(), json!(deployment_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployments/duplicate".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a deployment based on a template.
    /// 
    /// Use this endpoint with combination of
    /// [listTemplates](https://appwrite.io/docs/products/sites/templates) to find
    /// the template details.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_template_deployment(
        &self,
        site_id: impl Into<String>,
        repository: impl Into<String>,
        owner: impl Into<String>,
        root_directory: impl Into<String>,
        r#type: crate::enums::TemplateReferenceType,
        reference: impl Into<String>,
        activate: Option<bool>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        params.insert("repository".to_string(), json!(repository.into()));
        params.insert("owner".to_string(), json!(owner.into()));
        params.insert("rootDirectory".to_string(), json!(root_directory.into()));
        params.insert("type".to_string(), json!(r#type));
        params.insert("reference".to_string(), json!(reference.into()));
        if let Some(value) = activate {
            params.insert("activate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployments/template".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a deployment when a site is connected to VCS.
    /// 
    /// This endpoint lets you create deployment from a branch, commit, or a tag.
    pub async fn create_vcs_deployment(
        &self,
        site_id: impl Into<String>,
        r#type: crate::enums::VCSReferenceType,
        reference: impl Into<String>,
        activate: Option<bool>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), json!(r#type));
        params.insert("reference".to_string(), json!(reference.into()));
        if let Some(value) = activate {
            params.insert("activate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployments/vcs".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a site deployment by its unique ID.
    pub async fn get_deployment(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let params = HashMap::new();

        let path = "/sites/{siteId}/deployments/{deploymentId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a site deployment by its unique ID.
    pub async fn delete_deployment(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployments/{deploymentId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a site deployment content by its unique ID. The endpoint response
    /// return with a 'Content-Disposition: attachment' header that tells the
    /// browser to start downloading the file to user downloads directory.
    pub async fn get_deployment_download(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
        r#type: Option<crate::enums::DeploymentDownloadType>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }

        let path = "/sites/{siteId}/deployments/{deploymentId}/download".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Cancel an ongoing site deployment build. If the build is already in
    /// progress, it will be stopped and marked as canceled. If the build hasn't
    /// started yet, it will be marked as canceled without executing. You cannot
    /// cancel builds that have already completed (status 'ready') or failed. The
    /// response includes the final build status and details.
    pub async fn update_deployment_status(
        &self,
        site_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/deployments/{deploymentId}/status".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all site logs. You can use the query params to filter your
    /// results.
    pub async fn list_logs(
        &self,
        site_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ExecutionList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/sites/{siteId}/logs".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a site request log by its unique ID.
    pub async fn get_log(
        &self,
        site_id: impl Into<String>,
        log_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Execution> {
        let params = HashMap::new();

        let path = "/sites/{siteId}/logs/{logId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{logId}", &log_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a site log by its unique ID.
    pub async fn delete_log(
        &self,
        site_id: impl Into<String>,
        log_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/logs/{logId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{logId}", &log_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all variables of a specific site.
    pub async fn list_variables(
        &self,
        site_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::VariableList> {
        let params = HashMap::new();

        let path = "/sites/{siteId}/variables".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new site variable. These variables can be accessed during build
    /// and runtime (server-side rendering) as environment variables.
    pub async fn create_variable(
        &self,
        site_id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("value".to_string(), json!(value.into()));
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/variables".to_string().replace("{siteId}", &site_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a variable by its unique ID.
    pub async fn get_variable(
        &self,
        site_id: impl Into<String>,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Variable> {
        let params = HashMap::new();

        let path = "/sites/{siteId}/variables/{variableId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update variable by its unique ID.
    pub async fn update_variable(
        &self,
        site_id: impl Into<String>,
        variable_id: impl Into<String>,
        key: impl Into<String>,
        value: Option<&str>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/variables/{variableId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a variable by its unique ID.
    pub async fn delete_variable(
        &self,
        site_id: impl Into<String>,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/sites/{siteId}/variables/{variableId}".to_string().replace("{siteId}", &site_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Sites {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sites_creation() {
        let client = Client::new();
        let service = Sites::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
