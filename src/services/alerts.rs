//! Alerts service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Alerts service lists the alerts your monitors raised: when each fired,
/// the readings while it fired, and when it resolved.
#[derive(Debug, Clone)]
pub struct Alerts {
    client: Client,
}

impl Alerts {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of the alerts the project's monitors raised, open and resolved.
    /// An alert is created when a monitor starts firing and resolved when it
    /// recovers. Use queries to filter by monitor, status, type or resource, order
    /// by `$createdAt` for the latest first, and `Query.search('search', term)` to
    /// search alert and monitor IDs, names and metrics.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AlertList> {
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

        let path = "/alerts".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get an alert by its unique ID: the monitor that raised it, when it fired
    /// and resolved, and the readings while it fired.
    pub async fn get(
        &self,
        alert_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Alert> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let alert_id = alert_id.into();
        if alert_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"alertId\"",
                None,
                String::new(),
            ));
        }

        let path = "/alerts/{alertId}"
            .to_string()
            .replace("{alertId}", &Client::encode_path(&alert_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Acknowledge a firing alert, or undo that. Everyone who can read the alert
    /// sees who acknowledged it and when. The alert keeps firing until its monitor
    /// recovers; a resolved alert keeps its acknowledgement and cannot be changed.
    pub async fn update(
        &self,
        alert_id: impl Into<String>,
        acknowledged: bool,
    ) -> crate::error::Result<crate::models::Alert> {
        let mut params = HashMap::new();
        params.insert("acknowledged".to_string(), json!(acknowledged));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let alert_id = alert_id.into();
        if alert_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"alertId\"",
                None,
                String::new(),
            ));
        }

        let path = "/alerts/{alertId}"
            .to_string()
            .replace("{alertId}", &Client::encode_path(&alert_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Alerts {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerts_creation() {
        let client = Client::new();
        let service = Alerts::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
