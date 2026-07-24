//! Backups service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Backups service allows you to manage backup policies, archives, and
/// restorations for your project.
#[derive(Debug, Clone)]
pub struct Backups {
    client: Client,
}

impl Backups {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all archives for a project.
    pub async fn list_archives(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::BackupArchiveList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/archives".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new archive asynchronously for a project.
    pub async fn create_archive(
        &self,
        services: Vec<crate::enums::BackupServices>,
        resource_id: Option<&str>,
    ) -> crate::error::Result<crate::models::BackupArchive> {
        let mut params = HashMap::new();
        params.insert("services".to_string(), json!(services));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/archives".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a backup archive using it's ID.
    pub async fn get_archive(
        &self,
        archive_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::BackupArchive> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/archives/{archiveId}".to_string().replace("{archiveId}", &archive_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an existing archive for a project.
    pub async fn delete_archive(
        &self,
        archive_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/archives/{archiveId}".to_string().replace("{archiveId}", &archive_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// List all policies for a project.
    pub async fn list_policies(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::BackupPolicyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/policies".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new backup policy.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_policy(
        &self,
        policy_id: impl Into<String>,
        services: Vec<crate::enums::BackupServices>,
        retention: i64,
        schedule: impl Into<String>,
        name: Option<&str>,
        resource_id: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let mut params = HashMap::new();
        params.insert("policyId".to_string(), json!(policy_id.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        params.insert("services".to_string(), json!(services));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        params.insert("retention".to_string(), json!(retention));
        params.insert("schedule".to_string(), json!(schedule.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/policies".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a backup policy using it's ID.
    pub async fn get_policy(
        &self,
        policy_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/policies/{policyId}".to_string().replace("{policyId}", &policy_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Update an existing policy using it's ID.
    pub async fn update_policy(
        &self,
        policy_id: impl Into<String>,
        name: Option<&str>,
        retention: Option<i64>,
        schedule: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = retention {
            params.insert("retention".to_string(), json!(value));
        }
        if let Some(value) = schedule {
            params.insert("schedule".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/policies/{policyId}".to_string().replace("{policyId}", &policy_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a policy using it's ID.
    pub async fn delete_policy(
        &self,
        policy_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/policies/{policyId}".to_string().replace("{policyId}", &policy_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Create and trigger a new restoration for a backup on a project.
    /// 
    /// For a backup of one database, the restoration resolves its destination
    /// before it is queued. Pass `newResourceId` to restore into that database ID,
    /// including the archived database ID to overwrite it. When `newResourceId` is
    /// omitted, a new database ID is generated and returned in `options`.
    /// 
    /// The restoration migration records the archived database in `resourceId` and
    /// `resourceType`, and the resolved database in `destinationResourceId` and
    /// `destinationResourceType`. Database types are stored canonically as
    /// `database`, `documentsdb`, or `vectorsdb`. Project-wide restorations leave
    /// these fields empty because they do not have a single source or destination
    /// database.
    /// 
    /// To list every migration related to one database, use its canonical type in
    /// a nested `OR(AND(...), AND(...), AND(...))` across the root, parent, and
    /// destination relation pairs: `(resourceType, resourceId)`,
    /// `(parentResourceType, parentResourceId)`, and `(destinationResourceType,
    /// destinationResourceId)`. Legacy and TablesDB databases use `database`; the
    /// operational `resourceType` of a table migration is not rewritten to
    /// `tablesdb`.
    /// 
    /// When restoring a DocumentsDB or VectorsDB database to a new resource from a
    /// dedicated source, the restore provisions a fresh dedicated backing database
    /// at the source database's own specification.
    pub async fn create_restoration(
        &self,
        archive_id: impl Into<String>,
        services: Vec<crate::enums::BackupServices>,
        new_resource_id: Option<&str>,
        new_resource_name: Option<&str>,
    ) -> crate::error::Result<crate::models::BackupRestoration> {
        let mut params = HashMap::new();
        params.insert("archiveId".to_string(), json!(archive_id.into()));
        params.insert("services".to_string(), json!(services));
        if let Some(value) = new_resource_id {
            params.insert("newResourceId".to_string(), json!(value));
        }
        if let Some(value) = new_resource_name {
            params.insert("newResourceName".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/restoration".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// List all backup restorations for a project.
    pub async fn list_restorations(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::BackupRestorationList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/restorations".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Get the current status of a backup restoration.
    pub async fn get_restoration(
        &self,
        restoration_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::BackupRestoration> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/backups/restorations/{restorationId}".to_string().replace("{restorationId}", &restoration_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Backups {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backups_creation() {
        let client = Client::new();
        let service = Backups::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
