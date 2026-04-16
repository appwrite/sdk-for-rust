//! Webhooks service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Webhooks {
    client: Client,
}

impl Webhooks {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all webhooks belonging to the project. You can use the query
    /// params to filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::WebhookList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/webhooks".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new webhook. Use this endpoint to configure a URL that will
    /// receive events from Appwrite when specific events occur.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        webhook_id: impl Into<String>,
        url: impl Into<String>,
        name: impl Into<String>,
        events: impl IntoIterator<Item = impl Into<String>>,
        enabled: Option<bool>,
        tls: Option<bool>,
        auth_username: Option<&str>,
        auth_password: Option<&str>,
        secret: Option<&str>,
    ) -> crate::error::Result<crate::models::Webhook> {
        let mut params = HashMap::new();
        params.insert("webhookId".to_string(), json!(webhook_id.into()));
        params.insert("url".to_string(), json!(url.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("events".to_string(), json!(events.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = tls {
            params.insert("tls".to_string(), json!(value));
        }
        if let Some(value) = auth_username {
            params.insert("authUsername".to_string(), json!(value));
        }
        if let Some(value) = auth_password {
            params.insert("authPassword".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/webhooks".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a webhook by its unique ID. This endpoint returns details about a
    /// specific webhook configured for a project.
    pub async fn get(
        &self,
        webhook_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Webhook> {
        let params = HashMap::new();

        let path = "/webhooks/{webhookId}".to_string().replace("{webhookId}", &webhook_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a webhook by its unique ID. Use this endpoint to update the URL,
    /// events, or status of an existing webhook.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        webhook_id: impl Into<String>,
        name: impl Into<String>,
        url: impl Into<String>,
        events: impl IntoIterator<Item = impl Into<String>>,
        enabled: Option<bool>,
        tls: Option<bool>,
        auth_username: Option<&str>,
        auth_password: Option<&str>,
    ) -> crate::error::Result<crate::models::Webhook> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("url".to_string(), json!(url.into()));
        params.insert("events".to_string(), json!(events.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = tls {
            params.insert("tls".to_string(), json!(value));
        }
        if let Some(value) = auth_username {
            params.insert("authUsername".to_string(), json!(value));
        }
        if let Some(value) = auth_password {
            params.insert("authPassword".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/webhooks/{webhookId}".to_string().replace("{webhookId}", &webhook_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a webhook by its unique ID. Once deleted, the webhook will no longer
    /// receive project events.
    pub async fn delete(
        &self,
        webhook_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/webhooks/{webhookId}".to_string().replace("{webhookId}", &webhook_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the webhook signing key. This endpoint can be used to regenerate the
    /// signing key used to sign and validate payload deliveries for a specific
    /// webhook.
    pub async fn update_secret(
        &self,
        webhook_id: impl Into<String>,
        secret: Option<&str>,
    ) -> crate::error::Result<crate::models::Webhook> {
        let mut params = HashMap::new();
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/webhooks/{webhookId}/secret".to_string().replace("{webhookId}", &webhook_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Webhooks {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhooks_creation() {
        let client = Client::new();
        let service = Webhooks::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
