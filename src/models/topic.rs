//! Topic model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Topic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Topic {
    /// Topic ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Topic creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Topic update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// The name of the topic.
    #[serde(rename = "name")]
    pub name: String,
    /// Total count of email subscribers subscribed to the topic.
    #[serde(rename = "emailTotal")]
    pub email_total: i64,
    /// Total count of SMS subscribers subscribed to the topic.
    #[serde(rename = "smsTotal")]
    pub sms_total: i64,
    /// Total count of push subscribers subscribed to the topic.
    #[serde(rename = "pushTotal")]
    pub push_total: i64,
    /// Subscribe permissions.
    #[serde(rename = "subscribe")]
    pub subscribe: Vec<String>,
}

impl Topic {
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

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get email_total
    pub fn email_total(&self) -> &i64 {
        &self.email_total
    }

    /// Get sms_total
    pub fn sms_total(&self) -> &i64 {
        &self.sms_total
    }

    /// Get push_total
    pub fn push_total(&self) -> &i64 {
        &self.push_total
    }

    /// Get subscribe
    pub fn subscribe(&self) -> &Vec<String> {
        &self.subscribe
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_creation() {
        let _model = <Topic as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.email_total();
        let _ = _model.sms_total();
        let _ = _model.push_total();
        let _ = _model.subscribe();
    }

    #[test]
    fn test_topic_serialization() {
        let model = <Topic as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Topic, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
