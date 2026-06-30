//! Notifications service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Notifications {
    client: Client,
}

impl Notifications {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the list of notifications for the currently logged in console user. Use
    /// queries to filter the results by attributes such as read status, view
    /// timestamps, or creation date.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::NotificationList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/notifications".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Update a notification by its unique ID. Use the `read` parameter to mark
    /// the notification as read or unread.
    pub async fn update(
        &self,
        notification_id: impl Into<String>,
        read: bool,
    ) -> crate::error::Result<crate::models::Notification> {
        let mut params = HashMap::new();
        params.insert("read".to_string(), json!(read));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/notifications/{notificationId}".to_string().replace("{notificationId}", &notification_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Notifications {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications_creation() {
        let client = Client::new();
        let service = Notifications::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
