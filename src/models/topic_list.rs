//! TopicList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Topic list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TopicList {
    /// Total number of topics that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of topics.
    #[serde(rename = "topics")]
    pub topics: Vec<crate::models::Topic>,
}

impl TopicList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get topics
    pub fn topics(&self) -> &Vec<crate::models::Topic> {
        &self.topics
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_list_creation() {
        let _model = <TopicList as Default>::default();
        let _ = _model.total();
        let _ = _model.topics();
    }

    #[test]
    fn test_topic_list_serialization() {
        let model = <TopicList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<TopicList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
