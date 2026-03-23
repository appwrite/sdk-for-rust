//! TablesDB service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TablesDB {
    client: Client,
}

impl TablesDB {
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

        let path = "/tablesdb".to_string();

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

        let path = "/tablesdb".to_string();

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

        let path = "/tablesdb/transactions".to_string();

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

        let path = "/tablesdb/transactions".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a transaction by its unique ID.
    pub async fn get_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Transaction> {
        let params = HashMap::new();

        let path = "/tablesdb/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

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

        let path = "/tablesdb/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

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

        let path = "/tablesdb/transactions/{transactionId}".to_string().replace("{transactionId}", &transaction_id.into().to_string());

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

        let path = "/tablesdb/transactions/{transactionId}/operations".to_string().replace("{transactionId}", &transaction_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a database by its unique ID. This endpoint response returns a JSON
    /// object with the database metadata.
    pub async fn get(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Database> {
        let params = HashMap::new();

        let path = "/tablesdb/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

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

        let path = "/tablesdb/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

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

        let path = "/tablesdb/{databaseId}".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all tables that belong to the provided databaseId. You can
    /// use the search parameter to filter your results.
    pub async fn list_tables(
        &self,
        database_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::TableList> {
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

        let path = "/tablesdb/{databaseId}/tables".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Table. Before using this route, you should create a new
    /// database resource using either a [server
    /// integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable)
    /// API or directly from your database console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_table(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        name: impl Into<String>,
        permissions: Option<Vec<String>>,
        row_security: Option<bool>,
        enabled: Option<bool>,
        columns: Option<Vec<serde_json::Value>>,
        indexes: Option<Vec<serde_json::Value>>,
    ) -> crate::error::Result<crate::models::Table> {
        let mut params = HashMap::new();
        params.insert("tableId".to_string(), json!(table_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = row_security {
            params.insert("rowSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = columns {
            params.insert("columns".to_string(), json!(value));
        }
        if let Some(value) = indexes {
            params.insert("indexes".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables".to_string().replace("{databaseId}", &database_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a table by its unique ID. This endpoint response returns a JSON object
    /// with the table metadata.
    pub async fn get_table(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Table> {
        let params = HashMap::new();

        let path = "/tablesdb/{databaseId}/tables/{tableId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a table by its unique ID.
    pub async fn update_table(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
        row_security: Option<bool>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Table> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = row_security {
            params.insert("rowSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a table by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub async fn delete_table(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// List columns in the table.
    pub async fn list_columns(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a boolean column.
    pub async fn create_boolean_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<bool>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnBoolean> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/boolean".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a boolean column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_boolean_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<bool>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnBoolean> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/boolean/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a date time column according to the ISO 8601 standard.
    pub async fn create_datetime_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnDatetime> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/datetime".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a date time column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_datetime_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnDatetime> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/datetime/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an email column.
    pub async fn create_email_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnEmail> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/email".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an email column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_email_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnEmail> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/email/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an enumeration column. The `elements` param acts as a white-list of
    /// accepted values for this column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_enum_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnEnum> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/enum".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an enum column. Changing the `default` value will not update already
    /// existing rows.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_enum_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnEnum> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/enum/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a float column. Optionally, minimum and maximum values can be
    /// provided.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_float_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        min: Option<f64>,
        max: Option<f64>,
        default: Option<f64>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnFloat> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/float".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a float column. Changing the `default` value will not update already
    /// existing rows.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_float_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnFloat> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/float/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create an integer column. Optionally, minimum and maximum values can be
    /// provided.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_integer_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnInteger> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/integer".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an integer column. Changing the `default` value will not update
    /// already existing rows.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_integer_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnInteger> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/integer/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create IP address column.
    pub async fn create_ip_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnIp> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/ip".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an ip column. Changing the `default` value will not update already
    /// existing rows.
    pub async fn update_ip_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnIp> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/ip/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric line column.
    pub async fn create_line_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::ColumnLine> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/line".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a line column. Changing the `default` value will not update already
    /// existing rows.
    pub async fn update_line_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnLine> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/line/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a longtext column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_longtext_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnLongtext> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/longtext".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a longtext column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_longtext_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnLongtext> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/longtext/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a mediumtext column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_mediumtext_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnMediumtext> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/mediumtext".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a mediumtext column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_mediumtext_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnMediumtext> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/mediumtext/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric point column.
    pub async fn create_point_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::ColumnPoint> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/point".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a point column. Changing the `default` value will not update already
    /// existing rows.
    pub async fn update_point_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnPoint> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/point/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a geometric polygon column.
    pub async fn create_polygon_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::ColumnPolygon> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("required".to_string(), json!(required));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/polygon".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a polygon column. Changing the `default` value will not update
    /// already existing rows.
    pub async fn update_polygon_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<Vec<String>>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnPolygon> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/polygon/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create relationship column. [Learn more about relationship
    /// columns](https://appwrite.io/docs/databases-relationships#relationship-columns).
    #[allow(clippy::too_many_arguments)]
    pub async fn create_relationship_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        related_table_id: impl Into<String>,
        r#type: crate::enums::RelationshipType,
        two_way: Option<bool>,
        key: Option<&str>,
        two_way_key: Option<&str>,
        on_delete: Option<crate::enums::RelationMutate>,
    ) -> crate::error::Result<crate::models::ColumnRelationship> {
        let mut params = HashMap::new();
        params.insert("relatedTableId".to_string(), json!(related_table_id.into()));
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/relationship".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a string column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_string_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        size: i64,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnString> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/string".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a string column. Changing the `default` value will not update
    /// already existing rows.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_string_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        size: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnString> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/string/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a text column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_text_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnText> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/text".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a text column. Changing the `default` value will not update already
    /// existing rows.
    pub async fn update_text_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnText> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/text/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a URL column.
    pub async fn create_url_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnUrl> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/url".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an url column. Changing the `default` value will not update already
    /// existing rows.
    pub async fn update_url_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnUrl> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/url/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a varchar column.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_varchar_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        size: i64,
        required: bool,
        default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnVarchar> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/varchar".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a varchar column. Changing the `default` value will not update
    /// already existing rows.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_varchar_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        required: bool,
        default: Option<&str>,
        size: Option<i64>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnVarchar> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/varchar/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get column by ID.
    pub async fn get_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Deletes a column.
    pub async fn delete_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update relationship column. [Learn more about relationship
    /// columns](https://appwrite.io/docs/databases-relationships#relationship-columns).
    pub async fn update_relationship_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        on_delete: Option<crate::enums::RelationMutate>,
        new_key: Option<&str>,
    ) -> crate::error::Result<crate::models::ColumnRelationship> {
        let mut params = HashMap::new();
        if let Some(value) = on_delete {
            params.insert("onDelete".to_string(), json!(value));
        }
        if let Some(value) = new_key {
            params.insert("newKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/columns/{key}/relationship".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// List indexes on the table.
    pub async fn list_indexes(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ColumnIndexList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/tablesdb/{databaseId}/tables/{tableId}/indexes".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Creates an index on the columns listed. Your index should include all the
    /// columns you will query in a single request.
    /// Type can be `key`, `fulltext`, or `unique`.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_index(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
        r#type: crate::enums::IndexType,
        columns: impl IntoIterator<Item = impl Into<String>>,
        orders: Option<crate::enums::OrderBy>,
        lengths: Option<Vec<i64>>,
    ) -> crate::error::Result<crate::models::ColumnIndex> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), json!(key.into()));
        params.insert("type".to_string(), json!(r#type));
        params.insert("columns".to_string(), json!(columns.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = orders {
            params.insert("orders".to_string(), json!(value));
        }
        if let Some(value) = lengths {
            params.insert("lengths".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/indexes".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get index by ID.
    pub async fn get_index(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<crate::models::ColumnIndex> {
        let params = HashMap::new();

        let path = "/tablesdb/{databaseId}/tables/{tableId}/indexes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete an index.
    pub async fn delete_index(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        key: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/indexes/{key}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{key}", &key.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the user's rows in a given table. You can use the query
    /// params to filter your results.
    pub async fn list_rows(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
        total: Option<bool>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::RowList> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Row. Before using this route, you should create a new table
    /// resource using either a [server
    /// integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable)
    /// API or directly from your database console.
    pub async fn create_row(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        data: serde_json::Value,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
        let mut params = HashMap::new();
        params.insert("rowId".to_string(), json!(row_id.into()));
        params.insert("data".to_string(), json!(data));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create new Rows. Before using this route, you should create a new table
    /// resource using either a [server
    /// integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable)
    /// API or directly from your database console.
    pub async fn create_rows(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        rows: Vec<serde_json::Value>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::RowList> {
        let mut params = HashMap::new();
        params.insert("rows".to_string(), json!(rows));
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create or update Rows. Before using this route, you should create a new
    /// table resource using either a [server
    /// integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable)
    /// API or directly from your database console.
    pub async fn upsert_rows(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        rows: Vec<serde_json::Value>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::RowList> {
        let mut params = HashMap::new();
        params.insert("rows".to_string(), json!(rows));
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Update all rows that match your queries, if no queries are submitted then
    /// all rows are updated. You can pass only specific fields to be updated.
    pub async fn update_rows(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        data: Option<serde_json::Value>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::RowList> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Bulk delete rows using queries, if no queries are passed then all rows are
    /// deleted.
    pub async fn delete_rows(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::RowList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a row by its unique ID. This endpoint response returns a JSON object
    /// with the row data.
    pub async fn get_row(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        queries: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create or update a Row. Before using this route, you should create a new
    /// table resource using either a [server
    /// integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable)
    /// API or directly from your database console.
    pub async fn upsert_row(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        data: Option<serde_json::Value>,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Update a row by its unique ID. Using the patch method you can pass only
    /// specific fields that will get updated.
    pub async fn update_row(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        data: Option<serde_json::Value>,
        permissions: Option<Vec<String>>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a row by its unique ID.
    pub async fn delete_row(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        if let Some(value) = transaction_id {
            params.insert("transactionId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Decrement a specific column of a row by a given value.
    #[allow(clippy::too_many_arguments)]
    pub async fn decrement_row_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        column: impl Into<String>,
        value: Option<f64>,
        min: Option<f64>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}/{column}/decrement".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string()).replace("{column}", &column.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Increment a specific column of a row by a given value.
    #[allow(clippy::too_many_arguments)]
    pub async fn increment_row_column(
        &self,
        database_id: impl Into<String>,
        table_id: impl Into<String>,
        row_id: impl Into<String>,
        column: impl Into<String>,
        value: Option<f64>,
        max: Option<f64>,
        transaction_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Row> {
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

        let path = "/tablesdb/{databaseId}/tables/{tableId}/rows/{rowId}/{column}/increment".to_string().replace("{databaseId}", &database_id.into().to_string()).replace("{tableId}", &table_id.into().to_string()).replace("{rowId}", &row_id.into().to_string()).replace("{column}", &column.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for TablesDB {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tables_db_creation() {
        let client = Client::new();
        let service = TablesDB::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
