//! Functions service for Appwrite SDK

use crate::client::Client;
use crate::input_file::InputFile;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Functions Service allows you view, create and manage your Cloud
/// Functions.
#[derive(Debug, Clone)]
pub struct Functions {
    client: Client,
}

impl Functions {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the project's functions. You can use the query params to
    /// filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::FunctionList> {
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

        let path = "/functions".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new function. You can pass a list of
    /// [permissions](https://appwrite.io/docs/permissions) to allow different
    /// project users or team with access to execute the function using the client
    /// API.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        function_id: impl Into<String>,
        name: impl Into<String>,
        runtime: crate::enums::Runtime,
        execute: Option<Vec<String>>,
        events: Option<Vec<String>>,
        schedule: Option<&str>,
        timeout: Option<i64>,
        enabled: Option<bool>,
        logging: Option<bool>,
        entrypoint: Option<&str>,
        commands: Option<&str>,
        scopes: Option<Vec<crate::enums::Scopes>>,
        installation_id: Option<&str>,
        provider_repository_id: Option<&str>,
        provider_branch: Option<&str>,
        provider_silent_mode: Option<bool>,
        provider_root_directory: Option<&str>,
        build_specification: Option<&str>,
        runtime_specification: Option<&str>,
        deployment_retention: Option<i64>,
    ) -> crate::error::Result<crate::models::Function> {
        let mut params = HashMap::new();
        params.insert("functionId".to_string(), json!(function_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("runtime".to_string(), json!(runtime));
        if let Some(value) = execute {
            params.insert("execute".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = events {
            params.insert("events".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = schedule {
            params.insert("schedule".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = logging {
            params.insert("logging".to_string(), json!(value));
        }
        if let Some(value) = entrypoint {
            params.insert("entrypoint".to_string(), json!(value));
        }
        if let Some(value) = commands {
            params.insert("commands".to_string(), json!(value));
        }
        if let Some(value) = scopes {
            params.insert("scopes".to_string(), json!(value));
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

        let path = "/functions".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all runtimes that are currently active on your instance.
    pub async fn list_runtimes(
        &self,
    ) -> crate::error::Result<crate::models::RuntimeList> {
        let params = HashMap::new();

        let path = "/functions/runtimes".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List allowed function specifications for this instance.
    pub async fn list_specifications(
        &self,
    ) -> crate::error::Result<crate::models::SpecificationList> {
        let params = HashMap::new();

        let path = "/functions/specifications".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a function by its unique ID.
    pub async fn get(
        &self,
        function_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Function> {
        let params = HashMap::new();

        let path = "/functions/{functionId}".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update function by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        function_id: impl Into<String>,
        name: impl Into<String>,
        runtime: Option<crate::enums::Runtime>,
        execute: Option<Vec<String>>,
        events: Option<Vec<String>>,
        schedule: Option<&str>,
        timeout: Option<i64>,
        enabled: Option<bool>,
        logging: Option<bool>,
        entrypoint: Option<&str>,
        commands: Option<&str>,
        scopes: Option<Vec<crate::enums::Scopes>>,
        installation_id: Option<&str>,
        provider_repository_id: Option<&str>,
        provider_branch: Option<&str>,
        provider_silent_mode: Option<bool>,
        provider_root_directory: Option<&str>,
        build_specification: Option<&str>,
        runtime_specification: Option<&str>,
        deployment_retention: Option<i64>,
    ) -> crate::error::Result<crate::models::Function> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = runtime {
            params.insert("runtime".to_string(), json!(value));
        }
        if let Some(value) = execute {
            params.insert("execute".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = events {
            params.insert("events".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = schedule {
            params.insert("schedule".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = logging {
            params.insert("logging".to_string(), json!(value));
        }
        if let Some(value) = entrypoint {
            params.insert("entrypoint".to_string(), json!(value));
        }
        if let Some(value) = commands {
            params.insert("commands".to_string(), json!(value));
        }
        if let Some(value) = scopes {
            params.insert("scopes".to_string(), json!(value));
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

        let path = "/functions/{functionId}".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a function by its unique ID.
    pub async fn delete(
        &self,
        function_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the function active deployment. Use this endpoint to switch the code
    /// deployment that should be used when visitor opens your function.
    pub async fn update_function_deployment(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Function> {
        let mut params = HashMap::new();
        params.insert("deploymentId".to_string(), json!(deployment_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/deployment".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the function's code deployments. You can use the query
    /// params to filter your results.
    pub async fn list_deployments(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/deployments".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new function code deployment. Use this endpoint to upload a new
    /// version of your code function. To execute your newly uploaded code, you'll
    /// need to update the function's deployment to use your new deployment UID.
    /// 
    /// This endpoint accepts a tar.gz file compressed with your code. Make sure to
    /// include any dependencies your code has within the compressed file. You can
    /// learn more about code packaging in the [Appwrite Cloud Functions
    /// tutorial](https://appwrite.io/docs/functions).
    /// 
    /// Use the "command" param to set the entrypoint used to execute your code.
    pub async fn create_deployment(
        &self,
        function_id: impl Into<String>,
        code: InputFile,
        activate: bool,
        entrypoint: Option<&str>,
        commands: Option<&str>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        if let Some(value) = entrypoint {
            params.insert("entrypoint".to_string(), json!(value));
        }
        if let Some(value) = commands {
            params.insert("commands".to_string(), json!(value));
        }
        params.insert("activate".to_string(), json!(activate));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let path = "/functions/{functionId}/deployments".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.file_upload(&path, Some(api_headers), params, "code", code, None).await
    }

    /// Create a new build for an existing function deployment. This endpoint
    /// allows you to rebuild a deployment with the updated function configuration,
    /// including its entrypoint and build commands if they have been modified. The
    /// build process will be queued and executed asynchronously. The original
    /// deployment's code will be preserved and used for the new build.
    pub async fn create_duplicate_deployment(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
        build_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let mut params = HashMap::new();
        params.insert("deploymentId".to_string(), json!(deployment_id.into()));
        if let Some(value) = build_id {
            params.insert("buildId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/deployments/duplicate".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a deployment based on a template.
    /// 
    /// Use this endpoint with combination of
    /// [listTemplates](https://appwrite.io/docs/products/functions/templates) to
    /// find the template details.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_template_deployment(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/deployments/template".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a deployment when a function is connected to VCS.
    /// 
    /// This endpoint lets you create deployment from a branch, commit, or a tag.
    pub async fn create_vcs_deployment(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/deployments/vcs".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a function deployment by its unique ID.
    pub async fn get_deployment(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let params = HashMap::new();

        let path = "/functions/{functionId}/deployments/{deploymentId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a code deployment by its unique ID.
    pub async fn delete_deployment(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/deployments/{deploymentId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a function deployment content by its unique ID. The endpoint response
    /// return with a 'Content-Disposition: attachment' header that tells the
    /// browser to start downloading the file to user downloads directory.
    pub async fn get_deployment_download(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
        r#type: Option<crate::enums::DeploymentDownloadType>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }

        let path = "/functions/{functionId}/deployments/{deploymentId}/download".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Cancel an ongoing function deployment build. If the build is already in
    /// progress, it will be stopped and marked as canceled. If the build hasn't
    /// started yet, it will be marked as canceled without executing. You cannot
    /// cancel builds that have already completed (status 'ready') or failed. The
    /// response includes the final build status and details.
    pub async fn update_deployment_status(
        &self,
        function_id: impl Into<String>,
        deployment_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Deployment> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/deployments/{deploymentId}/status".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{deploymentId}", &deployment_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the current user function execution logs. You can use the
    /// query params to filter your results.
    pub async fn list_executions(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/executions".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Trigger a function execution. The returned object will return you the
    /// current execution status. You can ping the `Get Execution` endpoint to get
    /// updates on the current execution status. Once this endpoint is called, your
    /// function execution process will start asynchronously.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_execution(
        &self,
        function_id: impl Into<String>,
        body: Option<&str>,
        r#async: Option<bool>,
        path: Option<&str>,
        method: Option<crate::enums::ExecutionMethod>,
        headers: Option<serde_json::Value>,
        scheduled_at: Option<&str>,
    ) -> crate::error::Result<crate::models::Execution> {
        let mut params = HashMap::new();
        if let Some(value) = body {
            params.insert("body".to_string(), json!(value));
        }
        if let Some(value) = r#async {
            params.insert("async".to_string(), json!(value));
        }
        if let Some(value) = path {
            params.insert("path".to_string(), json!(value));
        }
        if let Some(value) = method {
            params.insert("method".to_string(), json!(value));
        }
        if let Some(value) = headers {
            params.insert("headers".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/executions".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a function execution log by its unique ID.
    pub async fn get_execution(
        &self,
        function_id: impl Into<String>,
        execution_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Execution> {
        let params = HashMap::new();

        let path = "/functions/{functionId}/executions/{executionId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{executionId}", &execution_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a function execution by its unique ID.
    pub async fn delete_execution(
        &self,
        function_id: impl Into<String>,
        execution_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/executions/{executionId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{executionId}", &execution_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all variables of a specific function.
    pub async fn list_variables(
        &self,
        function_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::VariableList> {
        let params = HashMap::new();

        let path = "/functions/{functionId}/variables".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new function environment variable. These variables can be accessed
    /// in the function at runtime as environment variables.
    pub async fn create_variable(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/variables".to_string().replace("{functionId}", &function_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a variable by its unique ID.
    pub async fn get_variable(
        &self,
        function_id: impl Into<String>,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Variable> {
        let params = HashMap::new();

        let path = "/functions/{functionId}/variables/{variableId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update variable by its unique ID.
    pub async fn update_variable(
        &self,
        function_id: impl Into<String>,
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

        let path = "/functions/{functionId}/variables/{variableId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a variable by its unique ID.
    pub async fn delete_variable(
        &self,
        function_id: impl Into<String>,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/functions/{functionId}/variables/{variableId}".to_string().replace("{functionId}", &function_id.into().to_string()).replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Functions {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions_creation() {
        let client = Client::new();
        let service = Functions::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
