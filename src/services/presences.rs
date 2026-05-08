//! Presences service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Presences {
    client: Client,
}

impl Presences {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List presence logs.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::PresenceList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }

        let path = "/presences".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a presence log by its unique ID.
    pub async fn get(
        &self,
        presence_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Presence> {
        let params = HashMap::new();

        let path = "/presences/{presenceId}".to_string().replace("{presenceId}", &presence_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create or update a presence log by its unique ID.
    pub async fn upsert(
        &self,
        presence_id: impl Into<String>,
        status: impl Into<String>,
        user_id: Option<&str>,
        permissions: Option<Vec<String>>,
        expires_at: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> crate::error::Result<crate::models::Presence> {
        let mut params = HashMap::new();
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        params.insert("status".to_string(), json!(status.into()));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = expires_at {
            params.insert("expiresAt".to_string(), json!(value));
        }
        if let Some(value) = metadata {
            params.insert("metadata".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/presences/{presenceId}".to_string().replace("{presenceId}", &presence_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Update a presence log by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_presence(
        &self,
        presence_id: impl Into<String>,
        user_id: Option<&str>,
        status: Option<&str>,
        expires_at: Option<&str>,
        metadata: Option<serde_json::Value>,
        permissions: Option<Vec<String>>,
        purge: Option<bool>,
    ) -> crate::error::Result<crate::models::Presence> {
        let mut params = HashMap::new();
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        if let Some(value) = status {
            params.insert("status".to_string(), json!(value));
        }
        if let Some(value) = expires_at {
            params.insert("expiresAt".to_string(), json!(value));
        }
        if let Some(value) = metadata {
            params.insert("metadata".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = purge {
            params.insert("purge".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/presences/{presenceId}".to_string().replace("{presenceId}", &presence_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a presence log by its unique ID.
    pub async fn delete(
        &self,
        presence_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/presences/{presenceId}".to_string().replace("{presenceId}", &presence_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Presences {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presences_creation() {
        let client = Client::new();
        let service = Presences::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
