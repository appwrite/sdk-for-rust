//! VectorsDB service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VectorsDB {
    client: Client,
}

impl VectorsDB {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all databases from the current Appwrite project. You can use
    /// the search parameter to filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::DatabaseList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new Database.
    pub async fn create(
        &self,
        database_id: impl Into<String>,
        name: impl Into<String>,
        enabled: Option<bool>,
        specification: Option<&str>,
        replicas: Option<i64>,
        sync_mode: Option<&str>,
    ) -> crate::error::Result<crate::models::Database> {
        let mut params = HashMap::new();
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = specification {
            params.insert("specification".to_string(), json!(value));
        }
        if let Some(value) = replicas {
            params.insert("replicas".to_string(), json!(value));
        }
        if let Some(value) = sync_mode {
            params.insert("syncMode".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the dedicated database specifications available on the current plan.
    /// Each specification reports its resource limits, pricing, and whether it is
    /// enabled for the organization.
    pub async fn list_specifications(
        &self,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseSpecificationList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/specifications".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// List transactions across all databases.
    pub async fn list_transactions(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::TransactionList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
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
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a transaction by its unique ID.
    pub async fn get_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions/{transactionId}"
            .to_string()
            .replace("{transactionId}", &transaction_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
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
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions/{transactionId}"
            .to_string()
            .replace("{transactionId}", &transaction_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a transaction by its unique ID.
    pub async fn delete_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions/{transactionId}"
            .to_string()
            .replace("{transactionId}", &transaction_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
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
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/transactions/{transactionId}/operations"
            .to_string()
            .replace("{transactionId}", &transaction_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a database by its unique ID. This endpoint response returns a JSON
    /// object with the database metadata.
    pub async fn get(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Database> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a database by its unique ID.
    pub async fn update(
        &self,
        database_id: impl Into<String>,
        name: impl Into<String>,
        enabled: Option<bool>,
        specification: Option<&str>,
        replicas: Option<i64>,
        sync_mode: Option<&str>,
    ) -> crate::error::Result<crate::models::Database> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = specification {
            params.insert("specification".to_string(), json!(value));
        }
        if let Some(value) = replicas {
            params.insert("replicas".to_string(), json!(value));
        }
        if let Some(value) = sync_mode {
            params.insert("syncMode".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a database by its unique ID. Only API keys with with databases.write
    /// scope can delete a database.
    pub async fn delete(&self, database_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a list of all collections that belong to the provided databaseId. You
    /// can use the search parameter to filter your results.
    pub async fn list_collections(
        &self,
        database_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VectorsdbCollectionList> {
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

        let path = "/vectorsdb/{databaseId}/collections"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new Collection. Before using this route, you should create a new
    /// database resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#documentsDBCreateCollection)
    /// API or directly from your database console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        name: impl Into<String>,
        dimension: i64,
        permissions: Option<Vec<String>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::VectorsdbCollection> {
        let mut params = HashMap::new();
        params.insert("collectionId".to_string(), json!(collection_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("dimension".to_string(), json!(dimension));
        if let Some(value) = permissions {
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = document_security {
            params.insert("documentSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a collection by its unique ID. This endpoint response returns a JSON
    /// object with the collection metadata.
    pub async fn get_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::VectorsdbCollection> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a collection by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_collection(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        name: impl Into<String>,
        dimension: Option<i64>,
        permissions: Option<Vec<String>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::VectorsdbCollection> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = dimension {
            params.insert("dimension".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = document_security {
            params.insert("documentSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
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

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
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
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new Document. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#documentsDBCreateCollection)
    /// API or directly from your database console.
    pub async fn create_document(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        document_id: impl Into<String>,
        data: serde_json::Value,
        permissions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Document> {
        let mut params = HashMap::new();
        params.insert("documentId".to_string(), json!(document_id.into()));
        params.insert("data".to_string(), json!(data));
        if let Some(value) = permissions {
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create new Documents. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#documentsDBCreateCollection)
    /// API or directly from your database console.
    pub async fn create_documents(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        documents: Vec<serde_json::Value>,
    ) -> crate::error::Result<crate::models::DocumentList> {
        let mut params = HashMap::new();
        params.insert("documents".to_string(), json!(documents));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create or update Documents. Before using this route, you should create a
    /// new collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#documentsDBCreateCollection)
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
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a list of all the user's documents in a given collection using a POST
    /// request. This behaves identically to the list documents endpoint but
    /// accepts the queries in the request body, allowing much larger `queries`
    /// arrays than can fit in a URL query string.
    pub async fn create_query(
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
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
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents/query"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{documentId}", &document_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create or update a Document. Before using this route, you should create a
    /// new collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#documentsDBCreateCollection)
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
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{documentId}", &document_id.into().to_string());

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{documentId}", &document_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
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

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{documentId}", &document_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
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
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/indexes"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
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
        r#type: crate::enums::VectorsDBIndexType,
        attributes: impl IntoIterator<Item = impl Into<String>>,
        orders: Option<Vec<crate::enums::OrderBy>>,
        lengths: Option<Vec<i64>>,
    ) -> crate::error::Result<crate::models::Index> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("type".to_string(), json!(r#type));
        params.insert(
            "attributes".to_string(),
            json!(attributes
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<String>>()),
        );
        if let Some(value) = orders {
            params.insert("orders".to_string(), json!(value));
        }
        if let Some(value) = lengths {
            params.insert("lengths".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/indexes"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get index by ID.
    pub async fn get_index(
        &self,
        database_id: impl Into<String>,
        collection_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<crate::models::Index> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/indexes/{key}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{key}", &key.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
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

        let path = "/vectorsdb/{databaseId}/collections/{collectionId}/indexes/{key}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{collectionId}", &collection_id.into().to_string())
            .replace("{key}", &key.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Trigger a manual failover for a dedicated database with high availability
    /// enabled. Promotes a replica to primary. The failover runs asynchronously;
    /// poll the database document for status updates. A database left
    /// mid-operation also accepts this call as a repair once nothing is driving
    /// the operation it is stuck in. Repairing a failover that did not finish, a
    /// `failed` database, a stranded upgrade or migrate, or a stranded compute
    /// resize additionally requires `targetReplicaId` to name the member to
    /// promote, because the default target may be the member that operation
    /// already promoted.
    pub async fn create_failover(
        &self,
        database_id: impl Into<String>,
        target_replica_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        if let Some(value) = target_replica_id {
            params.insert("targetReplicaId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/failovers"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the lifecycle operations recorded for a dedicated database, newest
    /// first. Every provision, update, restore, backup and replication action is
    /// recorded here with its outcome, including an attempt that was abandoned
    /// because another worker took over the database.
    pub async fn list_operations(
        &self,
        database_id: impl Into<String>,
        status: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseOperationList> {
        let mut params = HashMap::new();
        if let Some(value) = status {
            params.insert("status".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/operations"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get high availability status for a dedicated database. Returns replica
    /// statuses, replication lag, and sync mode.
    pub async fn get_replicas(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseReplicas> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/replicas"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get real-time health and status information for a dedicated database.
    /// Returns health status, readiness, uptime, connection info, replica status,
    /// and volume information.
    pub async fn get_status(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DatabaseStatus> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/vectorsdb/{databaseId}/status"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for VectorsDB {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectors_db_creation() {
        let client = Client::new();
        let service = VectorsDB::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
