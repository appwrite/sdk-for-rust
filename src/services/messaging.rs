//! Messaging service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Messaging service allows you to send messages to any provider type
/// (SMTP, push notification, SMS, etc.).
#[derive(Debug, Clone)]
pub struct Messaging {
    client: Client,
}

impl Messaging {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all messages from the current Appwrite project.
    pub async fn list_messages(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MessageList> {
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

        let path = "/messaging/messages".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new email message.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_email(
        &self,
        message_id: impl Into<String>,
        subject: impl Into<String>,
        content: impl Into<String>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        attachments: Option<Vec<String>>,
        draft: Option<bool>,
        html: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        params.insert("messageId".to_string(), json!(message_id.into()));
        params.insert("subject".to_string(), json!(subject.into()));
        params.insert("content".to_string(), json!(content.into()));
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = cc {
            params.insert("cc".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = bcc {
            params.insert("bcc".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = attachments {
            params.insert("attachments".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = html {
            params.insert("html".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/email".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an email message by its unique ID. This endpoint only works on
    /// messages that are in draft status. Messages that are already processing,
    /// sent, or failed cannot be updated.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_email(
        &self,
        message_id: impl Into<String>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        subject: Option<&str>,
        content: Option<&str>,
        draft: Option<bool>,
        html: Option<bool>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        scheduled_at: Option<&str>,
        attachments: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = subject {
            params.insert("subject".to_string(), json!(value));
        }
        if let Some(value) = content {
            params.insert("content".to_string(), json!(value));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = html {
            params.insert("html".to_string(), json!(value));
        }
        if let Some(value) = cc {
            params.insert("cc".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = bcc {
            params.insert("bcc".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        if let Some(value) = attachments {
            params.insert("attachments".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/email/{messageId}".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new push notification.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_push(
        &self,
        message_id: impl Into<String>,
        title: Option<&str>,
        body: Option<&str>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        data: Option<serde_json::Value>,
        action: Option<&str>,
        image: Option<&str>,
        icon: Option<&str>,
        sound: Option<&str>,
        color: Option<&str>,
        tag: Option<&str>,
        badge: Option<i64>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
        content_available: Option<bool>,
        critical: Option<bool>,
        priority: Option<crate::enums::MessagePriority>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        params.insert("messageId".to_string(), json!(message_id.into()));
        if let Some(value) = title {
            params.insert("title".to_string(), json!(value));
        }
        if let Some(value) = body {
            params.insert("body".to_string(), json!(value));
        }
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = data {
            params.insert("data".to_string(), json!(value));
        }
        if let Some(value) = action {
            params.insert("action".to_string(), json!(value));
        }
        if let Some(value) = image {
            params.insert("image".to_string(), json!(value));
        }
        if let Some(value) = icon {
            params.insert("icon".to_string(), json!(value));
        }
        if let Some(value) = sound {
            params.insert("sound".to_string(), json!(value));
        }
        if let Some(value) = color {
            params.insert("color".to_string(), json!(value));
        }
        if let Some(value) = tag {
            params.insert("tag".to_string(), json!(value));
        }
        if let Some(value) = badge {
            params.insert("badge".to_string(), json!(value));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        if let Some(value) = content_available {
            params.insert("contentAvailable".to_string(), json!(value));
        }
        if let Some(value) = critical {
            params.insert("critical".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/push".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a push notification by its unique ID. This endpoint only works on
    /// messages that are in draft status. Messages that are already processing,
    /// sent, or failed cannot be updated.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_push(
        &self,
        message_id: impl Into<String>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        title: Option<&str>,
        body: Option<&str>,
        data: Option<serde_json::Value>,
        action: Option<&str>,
        image: Option<&str>,
        icon: Option<&str>,
        sound: Option<&str>,
        color: Option<&str>,
        tag: Option<&str>,
        badge: Option<i64>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
        content_available: Option<bool>,
        critical: Option<bool>,
        priority: Option<crate::enums::MessagePriority>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = title {
            params.insert("title".to_string(), json!(value));
        }
        if let Some(value) = body {
            params.insert("body".to_string(), json!(value));
        }
        if let Some(value) = data {
            params.insert("data".to_string(), json!(value));
        }
        if let Some(value) = action {
            params.insert("action".to_string(), json!(value));
        }
        if let Some(value) = image {
            params.insert("image".to_string(), json!(value));
        }
        if let Some(value) = icon {
            params.insert("icon".to_string(), json!(value));
        }
        if let Some(value) = sound {
            params.insert("sound".to_string(), json!(value));
        }
        if let Some(value) = color {
            params.insert("color".to_string(), json!(value));
        }
        if let Some(value) = tag {
            params.insert("tag".to_string(), json!(value));
        }
        if let Some(value) = badge {
            params.insert("badge".to_string(), json!(value));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        if let Some(value) = content_available {
            params.insert("contentAvailable".to_string(), json!(value));
        }
        if let Some(value) = critical {
            params.insert("critical".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/push/{messageId}".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new SMS message.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_sms(
        &self,
        message_id: impl Into<String>,
        content: impl Into<String>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        params.insert("messageId".to_string(), json!(message_id.into()));
        params.insert("content".to_string(), json!(content.into()));
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/sms".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an SMS message by its unique ID. This endpoint only works on
    /// messages that are in draft status. Messages that are already processing,
    /// sent, or failed cannot be updated.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_sms(
        &self,
        message_id: impl Into<String>,
        topics: Option<Vec<String>>,
        users: Option<Vec<String>>,
        targets: Option<Vec<String>>,
        content: Option<&str>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> crate::error::Result<crate::models::Message> {
        let mut params = HashMap::new();
        if let Some(value) = topics {
            params.insert("topics".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = users {
            params.insert("users".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = targets {
            params.insert("targets".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = content {
            params.insert("content".to_string(), json!(value));
        }
        if let Some(value) = draft {
            params.insert("draft".to_string(), json!(value));
        }
        if let Some(value) = scheduled_at {
            params.insert("scheduledAt".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/sms/{messageId}".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a message by its unique ID.
    pub async fn get_message(
        &self,
        message_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Message> {
        let params = HashMap::new();

        let path = "/messaging/messages/{messageId}".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a message. If the message is not a draft or scheduled, but has been
    /// sent, this will not recall the message.
    pub async fn delete(
        &self,
        message_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/messages/{messageId}".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get the message activity logs listed by its unique ID.
    pub async fn list_message_logs(
        &self,
        message_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::LogList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/messaging/messages/{messageId}/logs".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of the targets associated with a message.
    pub async fn list_targets(
        &self,
        message_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::TargetList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/messaging/messages/{messageId}/targets".to_string().replace("{messageId}", &message_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of all providers from the current Appwrite project.
    pub async fn list_providers(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ProviderList> {
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

        let path = "/messaging/providers".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Apple Push Notification service provider.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_apns_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        auth_key: Option<&str>,
        auth_key_id: Option<&str>,
        team_id: Option<&str>,
        bundle_id: Option<&str>,
        sandbox: Option<bool>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = auth_key {
            params.insert("authKey".to_string(), json!(value));
        }
        if let Some(value) = auth_key_id {
            params.insert("authKeyId".to_string(), json!(value));
        }
        if let Some(value) = team_id {
            params.insert("teamId".to_string(), json!(value));
        }
        if let Some(value) = bundle_id {
            params.insert("bundleId".to_string(), json!(value));
        }
        if let Some(value) = sandbox {
            params.insert("sandbox".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/apns".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Apple Push Notification service provider by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_apns_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        auth_key: Option<&str>,
        auth_key_id: Option<&str>,
        team_id: Option<&str>,
        bundle_id: Option<&str>,
        sandbox: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = auth_key {
            params.insert("authKey".to_string(), json!(value));
        }
        if let Some(value) = auth_key_id {
            params.insert("authKeyId".to_string(), json!(value));
        }
        if let Some(value) = team_id {
            params.insert("teamId".to_string(), json!(value));
        }
        if let Some(value) = bundle_id {
            params.insert("bundleId".to_string(), json!(value));
        }
        if let Some(value) = sandbox {
            params.insert("sandbox".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/apns/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Firebase Cloud Messaging provider.
    pub async fn create_fcm_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        service_account_json: Option<serde_json::Value>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = service_account_json {
            params.insert("serviceAccountJSON".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/fcm".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Firebase Cloud Messaging provider by its unique ID.
    pub async fn update_fcm_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        service_account_json: Option<serde_json::Value>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = service_account_json {
            params.insert("serviceAccountJSON".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/fcm/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Mailgun provider.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_mailgun_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        api_key: Option<&str>,
        domain: Option<&str>,
        is_eu_region: Option<bool>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = is_eu_region {
            params.insert("isEuRegion".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/mailgun".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Mailgun provider by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_mailgun_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        api_key: Option<&str>,
        domain: Option<&str>,
        is_eu_region: Option<bool>,
        enabled: Option<bool>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = is_eu_region {
            params.insert("isEuRegion".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/mailgun/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new MSG91 provider.
    pub async fn create_msg91_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        template_id: Option<&str>,
        sender_id: Option<&str>,
        auth_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = template_id {
            params.insert("templateId".to_string(), json!(value));
        }
        if let Some(value) = sender_id {
            params.insert("senderId".to_string(), json!(value));
        }
        if let Some(value) = auth_key {
            params.insert("authKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/msg91".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a MSG91 provider by its unique ID.
    pub async fn update_msg91_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        template_id: Option<&str>,
        sender_id: Option<&str>,
        auth_key: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = template_id {
            params.insert("templateId".to_string(), json!(value));
        }
        if let Some(value) = sender_id {
            params.insert("senderId".to_string(), json!(value));
        }
        if let Some(value) = auth_key {
            params.insert("authKey".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/msg91/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Resend provider.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_resend_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/resend".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Resend provider by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_resend_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/resend/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Sendgrid provider.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_sendgrid_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/sendgrid".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Sendgrid provider by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_sendgrid_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/sendgrid/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new SMTP provider.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_smtp_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: Option<i64>,
        username: Option<&str>,
        password: Option<&str>,
        encryption: Option<crate::enums::SmtpEncryption>,
        auto_tls: Option<bool>,
        mailer: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("host".to_string(), json!(host.into()));
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = password {
            params.insert("password".to_string(), json!(value));
        }
        if let Some(value) = encryption {
            params.insert("encryption".to_string(), json!(value));
        }
        if let Some(value) = auto_tls {
            params.insert("autoTLS".to_string(), json!(value));
        }
        if let Some(value) = mailer {
            params.insert("mailer".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/smtp".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a SMTP provider by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_smtp_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        host: Option<&str>,
        port: Option<i64>,
        username: Option<&str>,
        password: Option<&str>,
        encryption: Option<crate::enums::SmtpEncryption>,
        auto_tls: Option<bool>,
        mailer: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = host {
            params.insert("host".to_string(), json!(value));
        }
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = password {
            params.insert("password".to_string(), json!(value));
        }
        if let Some(value) = encryption {
            params.insert("encryption".to_string(), json!(value));
        }
        if let Some(value) = auto_tls {
            params.insert("autoTLS".to_string(), json!(value));
        }
        if let Some(value) = mailer {
            params.insert("mailer".to_string(), json!(value));
        }
        if let Some(value) = from_name {
            params.insert("fromName".to_string(), json!(value));
        }
        if let Some(value) = from_email {
            params.insert("fromEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/smtp/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Telesign provider.
    pub async fn create_telesign_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        from: Option<&str>,
        customer_id: Option<&str>,
        api_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        if let Some(value) = customer_id {
            params.insert("customerId".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/telesign".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Telesign provider by its unique ID.
    pub async fn update_telesign_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        customer_id: Option<&str>,
        api_key: Option<&str>,
        from: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = customer_id {
            params.insert("customerId".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/telesign/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Textmagic provider.
    pub async fn create_textmagic_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        from: Option<&str>,
        username: Option<&str>,
        api_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/textmagic".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Textmagic provider by its unique ID.
    pub async fn update_textmagic_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        username: Option<&str>,
        api_key: Option<&str>,
        from: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/textmagic/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Twilio provider.
    pub async fn create_twilio_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        from: Option<&str>,
        account_sid: Option<&str>,
        auth_token: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        if let Some(value) = account_sid {
            params.insert("accountSid".to_string(), json!(value));
        }
        if let Some(value) = auth_token {
            params.insert("authToken".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/twilio".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Twilio provider by its unique ID.
    pub async fn update_twilio_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        account_sid: Option<&str>,
        auth_token: Option<&str>,
        from: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = account_sid {
            params.insert("accountSid".to_string(), json!(value));
        }
        if let Some(value) = auth_token {
            params.insert("authToken".to_string(), json!(value));
        }
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/twilio/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Vonage provider.
    pub async fn create_vonage_provider(
        &self,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        from: Option<&str>,
        api_key: Option<&str>,
        api_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = api_secret {
            params.insert("apiSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/vonage".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Vonage provider by its unique ID.
    pub async fn update_vonage_provider(
        &self,
        provider_id: impl Into<String>,
        name: Option<&str>,
        enabled: Option<bool>,
        api_key: Option<&str>,
        api_secret: Option<&str>,
        from: Option<&str>,
    ) -> crate::error::Result<crate::models::Provider> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = api_secret {
            params.insert("apiSecret".to_string(), json!(value));
        }
        if let Some(value) = from {
            params.insert("from".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/vonage/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a provider by its unique ID.
    pub async fn get_provider(
        &self,
        provider_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Provider> {
        let params = HashMap::new();

        let path = "/messaging/providers/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a provider by its unique ID.
    pub async fn delete_provider(
        &self,
        provider_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/providers/{providerId}".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get the provider activity logs listed by its unique ID.
    pub async fn list_provider_logs(
        &self,
        provider_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::LogList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/messaging/providers/{providerId}/logs".to_string().replace("{providerId}", &provider_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the subscriber activity logs listed by its unique ID.
    pub async fn list_subscriber_logs(
        &self,
        subscriber_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::LogList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/messaging/subscribers/{subscriberId}/logs".to_string().replace("{subscriberId}", &subscriber_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of all topics from the current Appwrite project.
    pub async fn list_topics(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::TopicList> {
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

        let path = "/messaging/topics".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new topic.
    pub async fn create_topic(
        &self,
        topic_id: impl Into<String>,
        name: impl Into<String>,
        subscribe: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Topic> {
        let mut params = HashMap::new();
        params.insert("topicId".to_string(), json!(topic_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = subscribe {
            params.insert("subscribe".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/topics".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a topic by its unique ID.
    pub async fn get_topic(
        &self,
        topic_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Topic> {
        let params = HashMap::new();

        let path = "/messaging/topics/{topicId}".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a topic by its unique ID.
    pub async fn update_topic(
        &self,
        topic_id: impl Into<String>,
        name: Option<&str>,
        subscribe: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Topic> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = subscribe {
            params.insert("subscribe".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/topics/{topicId}".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a topic by its unique ID.
    pub async fn delete_topic(
        &self,
        topic_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/topics/{topicId}".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get the topic activity logs listed by its unique ID.
    pub async fn list_topic_logs(
        &self,
        topic_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::LogList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/messaging/topics/{topicId}/logs".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of all subscribers from the current Appwrite project.
    pub async fn list_subscribers(
        &self,
        topic_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::SubscriberList> {
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

        let path = "/messaging/topics/{topicId}/subscribers".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new subscriber.
    pub async fn create_subscriber(
        &self,
        topic_id: impl Into<String>,
        subscriber_id: impl Into<String>,
        target_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Subscriber> {
        let mut params = HashMap::new();
        params.insert("subscriberId".to_string(), json!(subscriber_id.into()));
        params.insert("targetId".to_string(), json!(target_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/topics/{topicId}/subscribers".to_string().replace("{topicId}", &topic_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a subscriber by its unique ID.
    pub async fn get_subscriber(
        &self,
        topic_id: impl Into<String>,
        subscriber_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Subscriber> {
        let params = HashMap::new();

        let path = "/messaging/topics/{topicId}/subscribers/{subscriberId}".to_string().replace("{topicId}", &topic_id.into().to_string()).replace("{subscriberId}", &subscriber_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a subscriber by its unique ID.
    pub async fn delete_subscriber(
        &self,
        topic_id: impl Into<String>,
        subscriber_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/messaging/topics/{topicId}/subscribers/{subscriberId}".to_string().replace("{topicId}", &topic_id.into().to_string()).replace("{subscriberId}", &subscriber_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Messaging {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messaging_creation() {
        let client = Client::new();
        let service = Messaging::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
