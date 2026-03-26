//! SubscriberList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Subscriber list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SubscriberList {
    /// Total number of subscribers that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of subscribers.
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<crate::models::Subscriber>,
}

impl SubscriberList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get subscribers
    pub fn subscribers(&self) -> &Vec<crate::models::Subscriber> {
        &self.subscribers
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscriber_list_creation() {
        let _model = <SubscriberList as Default>::default();
        let _ = _model.total();
        let _ = _model.subscribers();
    }

    #[test]
    fn test_subscriber_list_serialization() {
        let model = <SubscriberList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<SubscriberList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
