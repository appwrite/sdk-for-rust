//! ActivityEventList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Activity event list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ActivityEventList {
    /// Total number of events that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of events.
    #[serde(rename = "events")]
    #[serde(default = "default_events")]
    pub events: Vec<crate::models::ActivityEvent>,
}

fn default_total() -> i64 {
    0
}

fn default_events() -> Vec<crate::models::ActivityEvent> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl ActivityEventList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get events
    pub fn events(&self) -> &Vec<crate::models::ActivityEvent> {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_event_list_creation() {
        let _model = <ActivityEventList as Default>::default();
        let _ = _model.total();
        let _ = _model.events();
    }

    #[test]
    fn test_activity_event_list_serialization() {
        let model = <ActivityEventList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ActivityEventList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
