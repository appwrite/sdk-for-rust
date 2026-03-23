//! Graphql service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The GraphQL API allows you to query and mutate your Appwrite server using
/// GraphQL.
#[derive(Debug, Clone)]
pub struct Graphql {
    client: Client,
}

impl Graphql {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Execute a GraphQL mutation.
    pub async fn query(
        &self,
        query: serde_json::Value,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        params.insert("query".to_string(), json!(query));
        let mut api_headers = HashMap::new();
        api_headers.insert("x-sdk-graphql".to_string(), "true".to_string());
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/graphql".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Execute a GraphQL mutation.
    pub async fn mutation(
        &self,
        query: serde_json::Value,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        params.insert("query".to_string(), json!(query));
        let mut api_headers = HashMap::new();
        api_headers.insert("x-sdk-graphql".to_string(), "true".to_string());
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/graphql/mutation".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Graphql {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphql_creation() {
        let client = Client::new();
        let service = Graphql::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
