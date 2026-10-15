//! Subscriber model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Subscriber
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Subscriber {
    /// Subscriber ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Subscriber creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Subscriber update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Target ID.
    #[serde(rename = "targetId")]
    #[serde(default = "default_target_id")]
    pub target_id: String,
    /// Target.
    #[serde(rename = "target")]
    pub target: crate::models::Target,
    /// Topic ID.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
    /// User Name.
    #[serde(rename = "userName")]
    #[serde(default = "default_user_name")]
    pub user_name: String,
    /// Topic ID.
    #[serde(rename = "topicId")]
    #[serde(default = "default_topic_id")]
    pub topic_id: String,
    /// The target provider type. Can be one of the following: `email`, `sms` or
    /// `push`.
    #[serde(rename = "providerType")]
    #[serde(default = "default_provider_type")]
    pub provider_type: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_target_id() -> String {
    "".to_string()
}

fn default_user_id() -> String {
    "".to_string()
}

fn default_user_name() -> String {
    "".to_string()
}

fn default_topic_id() -> String {
    "".to_string()
}

fn default_provider_type() -> String {
    "".to_string()
}

impl Subscriber {
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

    /// Get target_id
    pub fn target_id(&self) -> &String {
        &self.target_id
    }

    /// Get target
    pub fn target(&self) -> &crate::models::Target {
        &self.target
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get user_name
    pub fn user_name(&self) -> &String {
        &self.user_name
    }

    /// Get topic_id
    pub fn topic_id(&self) -> &String {
        &self.topic_id
    }

    /// Get provider_type
    pub fn provider_type(&self) -> &String {
        &self.provider_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscriber_creation() {
        let _model = <Subscriber as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.target_id();
        let _ = _model.target();
        let _ = _model.user_id();
        let _ = _model.user_name();
        let _ = _model.topic_id();
        let _ = _model.provider_type();
    }

    #[test]
    fn test_subscriber_serialization() {
        let model = <Subscriber as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Subscriber, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
