//! Embeddings service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Embeddings {
    client: Client,
}

impl Embeddings {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Generate vector embeddings for an array of text using the selected
    /// embedding model. Use the returned vectors to power semantic search and
    /// similarity queries against your vector collections.
    pub async fn create_text_embeddings(
        &self,
        texts: impl IntoIterator<Item = impl Into<String>>,
        model: Option<crate::enums::EmbeddingModel>,
    ) -> crate::error::Result<crate::models::EmbeddingList> {
        let mut params = HashMap::new();
        params.insert(
            "texts".to_string(),
            json!(texts.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
        );
        if let Some(value) = model {
            params.insert("model".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/embeddings/text".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Embeddings {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embeddings_creation() {
        let client = Client::new();
        let service = Embeddings::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
