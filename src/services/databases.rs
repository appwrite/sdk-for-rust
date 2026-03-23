//! Databases service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Databases service allows you to create structured collections of
/// documents, query and filter lists of documents
#[derive(Debug, Clone)]
pub struct Databases {
    client: Client,
}

impl Databases {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all databases from the current Appwrite project. You can use
    /// the search parameter to filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::DatabaseList> {
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

        let path = "/databases".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Database.
    pub async fn create(
        &self,
        database_id: impl Into<String>,
        name: impl Into<String>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Database> {
        let mut params = HashMap::new();
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// List transactions across all databases.
    pub async fn list_transactions(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::TransactionList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }

        let path = "/databases/transactions".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new transaction.
    pub async fn create_transaction(
        &self,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let mut params = HashMap::new();
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/transactions".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a transaction by its unique ID.
    pub async fn get_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let params = HashMap::new();

        let path = "/databases/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a transaction, to either commit or roll back its operations.
    pub async fn update_transaction(
        &self,
        transaction_id: impl Into<String>,
        commit: Option<bool>,
        rollback: Option<bool>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let mut params = HashMap::new();
        if let Some(value) = commit {
            params.insert("commit".to_string(), json!(value));
        }
        if let Some(value) = rollback {
            params.insert("rollback".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a transaction by its unique ID.
    pub async fn delete_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Create multiple operations in a single transaction.
    pub async fn create_operations(
        &self,
        transaction_id: impl Into<String>,
        operations: Option<Vec<serde_json::Value>>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let mut params = HashMap::new();
        if let Some(value) = operations {
            params.insert("operations".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/transactions/{transactionId}/operations".to_string().replace("{transactionId}", &transaction_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a database by its unique ID. This endpoint response returns a JSON
    /// object with the database metadata.
    pub async fn get(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Database> {
        let params = HashMap::new();

        let path = "/databases/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a database by its unique ID.
    pub async fn update(
        &self,
        database_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Database> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a database by its unique ID. Only API keys with with databases.write
    /// scope can delete a database.
    pub async fn delete(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all collections that belong to the provided databaseId. You
    /// can use the search parameter to filter your results.
    pub async fn list_collections(
        &self,
        database_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::CollectionList> {
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

        let path = "/databases/{databaseId}/collections".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Collection. Before using this route, you should create a new
    /// database resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        name: impl Into<String>,
        permissions: Option<Vec<String>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
        attributes: Option<Vec<serde_json::Value>>,
        indexes: Option<Vec<serde_json::Value>>,
    ) -> crate::error::Result<crate::models::Collection> {
        let mut params = HashMap::new();
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = document_security {
            params.insert("documentSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = attributes {
            params.insert("attributes".to_string(), json!(value));
        }
        if let Some(value) = indexes {
            params.insert("indexes".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a collection by its unique ID. This endpoint response returns a JSON
    /// object with the collection metadata.
    pub async fn get_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Collection> {
        let params = HashMap::new();

        let path = "/databases/{databaseId}/collections/{collectionId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a collection by its unique ID.
    pub async fn update_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Collection> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = document_security {
            params.insert("documentSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a collection by its unique ID. Only users with write permissions
    /// have access to delete this resource.
    pub async fn delete_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// List attributes in the collection.
    pub async fn list_attributes(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a boolean attribute.
    pub async fn create_boolean_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<bool>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeBoolean> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/boolean".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a boolean attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_boolean_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<bool>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeBoolean> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/boolean/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a date time attribute according to the ISO 8601 standard.
    pub async fn create_datetime_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeDatetime> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/datetime".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a date time attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_datetime_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeDatetime> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/datetime/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an email attribute.
    pub async fn create_email_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeEmail> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/email".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an email attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_email_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeEmail> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/email/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an enum attribute. The `elements` param acts as a white-list of
    /// accepted values for this attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_enum_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeEnum> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("elements".to_string(), json!(elements.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an enum attribute. Changing the `default` value will not update
    /// already existing documents.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_enum_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeEnum> {
        let mut params = HashMap::new();
        params.insert("elements".to_string(), json!(elements.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a float attribute. Optionally, minimum and maximum values can be
    /// provided.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_float_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        min: Option<f64>,
        max: Option<f64>,
        default: Option<f64>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeFloat> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = min {
            params.insert("min".to_string(), json!(value));
        }
        if let Some(value) = max {
            params.insert("max".to_string(), json!(value));
        }
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/float".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a float attribute. Changing the `default` value will not update
    /// already existing documents.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_float_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeFloat> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = min {
            params.insert("min".to_string(), json!(value));
        }
        if let Some(value) = max {
            params.insert("max".to_string(), json!(value));
        }
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/float/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an integer attribute. Optionally, minimum and maximum values can be
    /// provided.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_integer_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeInteger> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = min {
            params.insert("min".to_string(), json!(value));
        }
        if let Some(value) = max {
            params.insert("max".to_string(), json!(value));
        }
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/integer".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an integer attribute. Changing the `default` value will not update
    /// already existing documents.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_integer_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeInteger> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = min {
            params.insert("min".to_string(), json!(value));
        }
        if let Some(value) = max {
            params.insert("max".to_string(), json!(value));
        }
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/integer/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create IP address attribute.
    pub async fn create_ip_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeIp> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an ip attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_ip_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeIp> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric line attribute.
    pub async fn create_line_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::AttributeLine> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/line".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a line attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_line_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeLine> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/line/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a longtext attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_longtext_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeLongtext> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        if let Some(value) = encrypt {
            params.insert("encrypt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/longtext".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a longtext attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_longtext_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeLongtext> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/longtext/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a mediumtext attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_mediumtext_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeMediumtext> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        if let Some(value) = encrypt {
            params.insert("encrypt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/mediumtext".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a mediumtext attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_mediumtext_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeMediumtext> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/mediumtext/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric point attribute.
    pub async fn create_point_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::AttributePoint> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/point".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a point attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_point_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributePoint> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/point/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric polygon attribute.
    pub async fn create_polygon_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::AttributePolygon> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/polygon".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a polygon attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_polygon_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributePolygon> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/polygon/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create relationship attribute. [Learn more about relationship
    /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
    #[allow(clippy::too_many_arguments)]
    pub async fn create_relationship_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        related_collection_id: impl Into<String>,
        r#type: crate::enums::RelationshipType,
        two_way: Option<bool>,
        key: Option<&str>,
        two_way_key: Option<&str>,
        on_delete: Option<crate::enums::RelationMutate>,
    ) -> crate::error::Result<crate::models::AttributeRelationship> {
        let mut params = HashMap::new();
        params.insert("relatedCollectionId".to_string(), json!(related_collection_id.into()));
        params.insert("type".to_string(), json!(r#type));
        if let Some(value) = two_way {
            params.insert("twoWay".to_string(), json!(value));
        }
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = two_way_key {
            params.insert("twoWayKey".to_string(), json!(value));
        }
        if let Some(value) = on_delete {
            params.insert("onDelete".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/relationship".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update relationship attribute. [Learn more about relationship
    /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
    pub async fn update_relationship_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        on_delete: Option<crate::enums::RelationMutate>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeRelationship> {
        let mut params = HashMap::new();
        if let Some(value) = on_delete {
            params.insert("onDelete".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/relationship/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a string attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_string_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        size: i64,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeString> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("size".to_string(), json!(size));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        if let Some(value) = encrypt {
            params.insert("encrypt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/string".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a string attribute. Changing the `default` value will not update
    /// already existing documents.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_string_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        size: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeString> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = size {
            params.insert("size".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/string/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a text attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_text_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeText> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        if let Some(value) = encrypt {
            params.insert("encrypt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/text".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a text attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_text_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeText> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/text/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a URL attribute.
    pub async fn create_url_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeUrl> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/url".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an url attribute. Changing the `default` value will not update
    /// already existing documents.
    pub async fn update_url_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeUrl> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/url/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a varchar attribute.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_varchar_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        size: i64,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::AttributeVarchar> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("size".to_string(), json!(size));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = array {
            params.insert("array".to_string(), json!(value));
        }
        if let Some(value) = encrypt {
            params.insert("encrypt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/varchar".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a varchar attribute. Changing the `default` value will not update
    /// already existing documents.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_varchar_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        size: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::AttributeVarchar> {
        let mut params = HashMap::new();
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        if let Some(value) = size {
            params.insert("size".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/varchar/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get attribute by ID.
    pub async fn get_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Deletes an attribute.
    pub async fn delete_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the user's documents in a given collection. You can use
    /// the query params to filter your results.
    pub async fn list_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
        total: Option<bool>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Document. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    pub async fn create_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        data: serde_json::Value,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        params.insert("documentId".to_string(), json!(document_id.into()));
        params.insert("data".to_string(), json!(data));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create new Documents. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    pub async fn create_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        documents: Vec<serde_json::Value>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        params.insert("documents".to_string(), json!(documents));
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create or update Documents. Before using this route, you should create a
    /// new collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    pub async fn upsert_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        documents: Vec<serde_json::Value>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        params.insert("documents".to_string(), json!(documents));
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Update all documents that match your queries, if no queries are submitted
    /// then all documents are updated. You can pass only specific fields to be
    /// updated.
    pub async fn update_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        data: Option<serde_json::Value>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        if let Some(value) = data {
            params.insert("data".to_string(), json!(value));
        }
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Bulk delete documents using queries, if no queries are passed then all
    /// documents are deleted.
    pub async fn delete_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a document by its unique ID. This endpoint response returns a JSON
    /// object with the document data.
    pub async fn get_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create or update a Document. Before using this route, you should create a
    /// new collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    pub async fn upsert_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        data: Option<serde_json::Value>,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        if let Some(value) = data {
            params.insert("data".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Update a document by its unique ID. Using the patch method you can pass
    /// only specific fields that will get updated.
    pub async fn update_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        data: Option<serde_json::Value>,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        if let Some(value) = data {
            params.insert("data".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a document by its unique ID.
    pub async fn delete_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Decrement a specific attribute of a document by a given value.
    #[allow(clippy::too_many_arguments)]
    pub async fn decrement_document_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        attribute: impl Into<String>,
        value: Option<f64>,
        min: Option<f64>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        if let Some(value) = min {
            params.insert("min".to_string(), json!(value));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}/{attribute}/decrement".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string()).replace("{attribute}", &attribute.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Increment a specific attribute of a document by a given value.
    #[allow(clippy::too_many_arguments)]
    pub async fn increment_document_attribute(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        attribute: impl Into<String>,
        value: Option<f64>,
        max: Option<f64>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        if let Some(value) = max {
            params.insert("max".to_string(), json!(value));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}/{attribute}/increment".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{documentId}", &document_id.into().to_string()).replace("{attribute}", &attribute.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// List indexes in the collection.
    pub async fn list_indexes(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::IndexList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/databases/{databaseId}/collections/{collectionId}/indexes".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Creates an index on the attributes listed. Your index should include all
    /// the attributes you will query in a single request.
    /// Attributes can be `key`, `fulltext`, and `unique`.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_index(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
        r#type: crate::enums::IndexType,
        attributes: impl IntoIterator<Item = impl Into<String>>,
        orders: Option<crate::enums::OrderBy>,
        lengths: Option<Vec<i64>>,
    ) -> crate::error::Result<crate::models::Index> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("type".to_string(), json!(r#type));
        params.insert("attributes".to_string(), json!(attributes.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = orders {
            params.insert("orders".to_string(), json!(value));
        }
        if let Some(value) = lengths {
            params.insert("lengths".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/indexes".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get an index by its unique ID.
    pub async fn get_index(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<crate::models::Index> {
        let params = HashMap::new();

        let path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete an index.
    pub async fn delete_index(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{collectionId}", &collection_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Databases {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_databases_creation() {
        let client = Client::new();
        let service = Databases::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
