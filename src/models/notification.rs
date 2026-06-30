//! Notification model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Notification {
    /// Notification ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Notification creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Notification update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Stable message ID used for dedup.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Notification type: info, warning, error.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Channel: email, sms, push, console, webhook.
    #[serde(rename = "channel")]
    pub channel: String,
    /// Resource type this notification is addressed to.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Resource ID this notification is addressed to.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Parent resource type for the notification.
    #[serde(rename = "parentResourceType")]
    pub parent_resource_type: String,
    /// Parent resource ID for the notification.
    #[serde(rename = "parentResourceId")]
    pub parent_resource_id: String,
    /// Project the notification pertains to.
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Notification title.
    #[serde(rename = "title")]
    pub title: String,
    /// Notification body.
    #[serde(rename = "body")]
    pub body: String,
    /// Whether the notification has been read.
    #[serde(rename = "read")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// First time the notification was viewed from a notification logo.
    #[serde(rename = "firstSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// Most recent time the notification was viewed from a notification logo.
    #[serde(rename = "lastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

impl Notification {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Set message_id
    pub fn set_message_id(mut self, message_id: String) -> Self {
        self.message_id = Some(message_id);
        self
    }

    /// Get message_id
    pub fn message_id(&self) -> Option<&String> {
        self.message_id.as_ref()
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get channel
    pub fn channel(&self) -> &String {
        &self.channel
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get parent_resource_type
    pub fn parent_resource_type(&self) -> &String {
        &self.parent_resource_type
    }

    /// Get parent_resource_id
    pub fn parent_resource_id(&self) -> &String {
        &self.parent_resource_id
    }

    /// Set project_id
    pub fn set_project_id(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Get project_id
    pub fn project_id(&self) -> Option<&String> {
        self.project_id.as_ref()
    }

    /// Get title
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get body
    pub fn body(&self) -> &String {
        &self.body
    }

    /// Set read
    pub fn set_read(mut self, read: bool) -> Self {
        self.read = Some(read);
        self
    }

    /// Get read
    pub fn read(&self) -> Option<&bool> {
        self.read.as_ref()
    }

    /// Set first_seen
    pub fn set_first_seen(mut self, first_seen: String) -> Self {
        self.first_seen = Some(first_seen);
        self
    }

    /// Get first_seen
    pub fn first_seen(&self) -> Option<&String> {
        self.first_seen.as_ref()
    }

    /// Set last_seen
    pub fn set_last_seen(mut self, last_seen: String) -> Self {
        self.last_seen = Some(last_seen);
        self
    }

    /// Get last_seen
    pub fn last_seen(&self) -> Option<&String> {
        self.last_seen.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let _model = <Notification as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.r#type();
        let _ = _model.channel();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.parent_resource_type();
        let _ = _model.parent_resource_id();
        let _ = _model.title();
        let _ = _model.body();
    }

    #[test]
    fn test_notification_serialization() {
        let model = <Notification as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Notification, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
