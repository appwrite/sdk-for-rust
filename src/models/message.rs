//! Message model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Message {
    /// Message ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Message provider type.
    #[serde(rename = "providerType")]
    pub provider_type: String,
    /// Topic IDs set as recipients.
    #[serde(rename = "topics")]
    pub topics: Vec<String>,
    /// User IDs set as recipients.
    #[serde(rename = "users")]
    pub users: Vec<String>,
    /// Target IDs set as recipients.
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
    /// The scheduled time for message.
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// The time when the message was delivered.
    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    /// Delivery errors if any.
    #[serde(rename = "deliveryErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_errors: Option<Vec<String>>,
    /// Number of recipients the message was delivered to.
    #[serde(rename = "deliveredTotal")]
    pub delivered_total: i64,
    /// Data of the message.
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    /// Status of delivery.
    #[serde(rename = "status")]
    pub status: crate::enums::MessageStatus,
}

impl Message {
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

    /// Get provider_type
    pub fn provider_type(&self) -> &String {
        &self.provider_type
    }

    /// Get topics
    pub fn topics(&self) -> &Vec<String> {
        &self.topics
    }

    /// Get users
    pub fn users(&self) -> &Vec<String> {
        &self.users
    }

    /// Get targets
    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    /// Set scheduled_at
    pub fn set_scheduled_at(mut self, scheduled_at: String) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }

    /// Get scheduled_at
    pub fn scheduled_at(&self) -> Option<&String> {
        self.scheduled_at.as_ref()
    }

    /// Set delivered_at
    pub fn set_delivered_at(mut self, delivered_at: String) -> Self {
        self.delivered_at = Some(delivered_at);
        self
    }

    /// Get delivered_at
    pub fn delivered_at(&self) -> Option<&String> {
        self.delivered_at.as_ref()
    }

    /// Set delivery_errors
    pub fn set_delivery_errors(mut self, delivery_errors: Vec<String>) -> Self {
        self.delivery_errors = Some(delivery_errors);
        self
    }

    /// Get delivery_errors
    pub fn delivery_errors(&self) -> Option<&Vec<String>> {
        self.delivery_errors.as_ref()
    }

    /// Get delivered_total
    pub fn delivered_total(&self) -> &i64 {
        &self.delivered_total
    }

    /// Get data
    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::MessageStatus {
        &self.status
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let _model = <Message as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.provider_type();
        let _ = _model.topics();
        let _ = _model.users();
        let _ = _model.targets();
        let _ = _model.delivered_total();
        let _ = _model.data();
        let _ = _model.status();
    }

    #[test]
    fn test_message_serialization() {
        let model = <Message as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Message, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
