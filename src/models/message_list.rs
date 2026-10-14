//! MessageList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Message list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MessageList {
    /// Total number of messages that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of messages.
    #[serde(rename = "messages")]
    #[serde(default = "default_messages")]
    pub messages: Vec<crate::models::Message>,
}

fn default_total() -> i64 {
    0
}

fn default_messages() -> Vec<crate::models::Message> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl MessageList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get messages
    pub fn messages(&self) -> &Vec<crate::models::Message> {
        &self.messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_list_creation() {
        let _model = <MessageList as Default>::default();
        let _ = _model.total();
        let _ = _model.messages();
    }

    #[test]
    fn test_message_list_serialization() {
        let model = <MessageList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MessageList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
