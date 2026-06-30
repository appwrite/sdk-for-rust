//! Organization service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Organization service allows you to manage organization-level projects.
#[derive(Debug, Clone)]
pub struct Organization {
    client: Client,
}

impl Organization {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all API keys from the current organization.
    pub async fn list_keys(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::KeyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/keys".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new organization API key.
    pub async fn create_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::OrganizationKeyScopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("keyId".to_string(), json!(key_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/keys".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a key by its unique ID. This endpoint returns details about a specific
    /// API key in your organization including its scopes.
    pub async fn get_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Key> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Update a key by its unique ID. Use this endpoint to update the name,
    /// scopes, or expiration time of an API key.
    pub async fn update_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::OrganizationKeyScopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a key by its unique ID. Once deleted, the key can no longer be used
    /// to authenticate API calls.
    pub async fn delete_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/organization/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all projects. You can use the query params to filter your
    /// results.
    pub async fn list_projects(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ProjectList> {
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
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/projects".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new project.
    pub async fn create_project(
        &self,
        project_id: impl Into<String>,
        name: impl Into<String>,
        region: Option<crate::enums::Region>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("projectId".to_string(), json!(project_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = region {
            params.insert("region".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/projects".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a project.
    pub async fn get_project(
        &self,
        project_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Project> {
        let params = HashMap::new();

        let path = "/organization/projects/{projectId}".to_string().replace("{projectId}", &project_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a project by its unique ID.
    pub async fn update_project(
        &self,
        project_id: impl Into<String>,
        name: impl Into<String>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/organization/projects/{projectId}".to_string().replace("{projectId}", &project_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a project by its unique ID.
    pub async fn delete_project(
        &self,
        project_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/organization/projects/{projectId}".to_string().replace("{projectId}", &project_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Organization {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_creation() {
        let client = Client::new();
        let service = Organization::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
