//! Migrations service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Migrations service allows you to migrate third-party data to your
/// Appwrite project.
#[derive(Debug, Clone)]
pub struct Migrations {
    client: Client,
}

impl Migrations {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all migrations in the current project. This endpoint returns a list of
    /// all migrations including their status, progress, and any errors that
    /// occurred during the migration process.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MigrationList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Migrate data from another Appwrite project to your current project. This
    /// endpoint allows you to migrate resources like databases, collections,
    /// documents, users, and files from an existing Appwrite project.
    pub async fn create_appwrite_migration(
        &self,
        resources: Vec<crate::enums::AppwriteMigrationResource>,
        endpoint: impl Into<String>,
        project_id: impl Into<String>,
        api_key: impl Into<String>,
        on_duplicate: Option<crate::enums::OnDuplicate>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("resources".to_string(), json!(resources));
        params.insert("endpoint".to_string(), json!(endpoint.into()));
        params.insert("projectId".to_string(), json!(project_id.into()));
        params.insert("apiKey".to_string(), json!(api_key.into()));
        if let Some(value) = on_duplicate {
            params.insert("onDuplicate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/appwrite".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Export documents to a CSV file from your Appwrite database. This endpoint
    /// allows you to export documents to a CSV file stored in a secure internal
    /// bucket. You'll receive an email with a download link when the export is
    /// complete.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_csv_export(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        filename: impl Into<String>,
        columns: Option<Vec<String>>,
        queries: Option<Vec<String>>,
        delimiter: Option<&str>,
        enclosure: Option<&str>,
        escape: Option<&str>,
        header: Option<bool>,
        notify: Option<bool>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        params.insert("filename".to_string(), json!(filename.into()));
        if let Some(value) = columns {
            params.insert(
                "columns".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = delimiter {
            params.insert("delimiter".to_string(), json!(value));
        }
        if let Some(value) = enclosure {
            params.insert("enclosure".to_string(), json!(value));
        }
        if let Some(value) = escape {
            params.insert("escape".to_string(), json!(value));
        }
        if let Some(value) = header {
            params.insert("header".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert("notify".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/csv/exports".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Import documents from a CSV file into your Appwrite database. This endpoint
    /// allows you to import documents from a CSV file uploaded to Appwrite Storage
    /// bucket.
    pub async fn create_csv_import(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        internal_file: Option<bool>,
        on_duplicate: Option<crate::enums::OnDuplicate>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("bucketId".to_string(), json!(bucket_id.into()));
        params.insert("fileId".to_string(), json!(file_id.into()));
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        if let Some(value) = internal_file {
            params.insert("internalFile".to_string(), json!(value));
        }
        if let Some(value) = on_duplicate {
            params.insert("onDuplicate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/csv/imports".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Migrate data from a Firebase project to your Appwrite project. This
    /// endpoint allows you to migrate resources like authentication and other
    /// supported services from a Firebase project.
    pub async fn create_firebase_migration(
        &self,
        resources: Vec<crate::enums::FirebaseMigrationResource>,
        service_account: impl Into<String>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("resources".to_string(), json!(resources));
        params.insert("serviceAccount".to_string(), json!(service_account.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/firebase".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Export documents to a JSON file from your Appwrite database. This endpoint
    /// allows you to export documents to a JSON file stored in a secure internal
    /// bucket. You'll receive an email with a download link when the export is
    /// complete.
    pub async fn create_json_export(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        filename: impl Into<String>,
        columns: Option<Vec<String>>,
        queries: Option<Vec<String>>,
        notify: Option<bool>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        params.insert("filename".to_string(), json!(filename.into()));
        if let Some(value) = columns {
            params.insert(
                "columns".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = notify {
            params.insert("notify".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/json/exports".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Import documents from a JSON file into your Appwrite database. This
    /// endpoint allows you to import documents from a JSON file uploaded to
    /// Appwrite Storage bucket.
    pub async fn create_json_import(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        internal_file: Option<bool>,
        on_duplicate: Option<crate::enums::OnDuplicate>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("bucketId".to_string(), json!(bucket_id.into()));
        params.insert("fileId".to_string(), json!(file_id.into()));
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        if let Some(value) = internal_file {
            params.insert("internalFile".to_string(), json!(value));
        }
        if let Some(value) = on_duplicate {
            params.insert("onDuplicate".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/json/imports".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Migrate data from an NHost project to your Appwrite project. This endpoint
    /// allows you to migrate resources like authentication, databases, and other
    /// supported services from an NHost project.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_n_host_migration(
        &self,
        resources: Vec<crate::enums::NHostMigrationResource>,
        subdomain: impl Into<String>,
        region: impl Into<String>,
        admin_secret: impl Into<String>,
        database: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
        port: Option<i64>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("resources".to_string(), json!(resources));
        params.insert("subdomain".to_string(), json!(subdomain.into()));
        params.insert("region".to_string(), json!(region.into()));
        params.insert("adminSecret".to_string(), json!(admin_secret.into()));
        params.insert("database".to_string(), json!(database.into()));
        params.insert("username".to_string(), json!(username.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/nhost".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Migrate data from a Supabase project to your Appwrite project. This
    /// endpoint allows you to migrate resources like authentication, databases,
    /// and other supported services from a Supabase project.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_supabase_migration(
        &self,
        resources: Vec<crate::enums::SupabaseMigrationResource>,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        database_host: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
        port: Option<i64>,
    ) -> crate::error::Result<crate::models::Migration> {
        let mut params = HashMap::new();
        params.insert("resources".to_string(), json!(resources));
        params.insert("endpoint".to_string(), json!(endpoint.into()));
        params.insert("apiKey".to_string(), json!(api_key.into()));
        params.insert("databaseHost".to_string(), json!(database_host.into()));
        params.insert("username".to_string(), json!(username.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/migrations/supabase".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a migration by its unique ID. This endpoint returns detailed
    /// information about a specific migration including its current status,
    /// progress, and any errors that occurred during the migration process.
    pub async fn get(
        &self,
        migration_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Migration> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let migration_id = migration_id.into();
        if migration_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"migrationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/migrations/{migrationId}"
            .to_string()
            .replace("{migrationId}", &Client::encode_path(&migration_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Retry a failed migration. This endpoint allows you to retry a migration
    /// that has previously failed.
    pub async fn retry(
        &self,
        migration_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Migration> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let migration_id = migration_id.into();
        if migration_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"migrationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/migrations/{migrationId}"
            .to_string()
            .replace("{migrationId}", &Client::encode_path(&migration_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a migration by its unique ID. This endpoint allows you to remove a
    /// migration from your project's migration history.
    pub async fn delete(&self, migration_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let migration_id = migration_id.into();
        if migration_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"migrationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/migrations/{migrationId}"
            .to_string()
            .replace("{migrationId}", &Client::encode_path(&migration_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Migrations {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_creation() {
        let client = Client::new();
        let service = Migrations::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
