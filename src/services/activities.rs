//! Activities service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Activities {
    client: Client,
}

impl Activities {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all events for selected filters.
    pub async fn list_events(
        &self,
        queries: Option<&str>,
    ) -> crate::error::Result<crate::models::ActivityEventList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value));
        }

        let path = "/activities/events".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get event by ID.
    pub async fn get_event(
        &self,
        event_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::ActivityEvent> {
        let params = HashMap::new();

        let path = "/activities/events/{eventId}".to_string().replace("{eventId}", &event_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Activities {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activities_creation() {
        let client = Client::new();
        let service = Activities::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
