//! Tokens service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tokens {
    client: Client,
}

impl Tokens {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all the tokens created for a specific file or bucket. You can use the
    /// query params to filter your results.
    pub async fn list(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ResourceTokenList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/tokens/buckets/{bucketId}/files/{fileId}".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new token. A token is linked to a file. Token can be passed as a
    /// request URL search parameter.
    pub async fn create_file_token(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::ResourceToken> {
        let mut params = HashMap::new();
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tokens/buckets/{bucketId}/files/{fileId}".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a token by its unique ID.
    pub async fn get(
        &self,
        token_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::ResourceToken> {
        let params = HashMap::new();

        let path = "/tokens/{tokenId}".to_string().replace("{tokenId}", &token_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a token by its unique ID. Use this endpoint to update a token's
    /// expiry date.
    pub async fn update(
        &self,
        token_id: impl Into<String>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::ResourceToken> {
        let mut params = HashMap::new();
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tokens/{tokenId}".to_string().replace("{tokenId}", &token_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a token by its unique ID.
    pub async fn delete(
        &self,
        token_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/tokens/{tokenId}".to_string().replace("{tokenId}", &token_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Tokens {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_creation() {
        let client = Client::new();
        let service = Tokens::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
