//! HealthQueue model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Health Queue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthQueue {
    /// Amount of actions in the queue.
    #[serde(rename = "size")]
    pub size: i64,
}

impl HealthQueue {
    /// Get size
    pub fn size(&self) -> &i64 {
        &self.size
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_queue_creation() {
        let _model = <HealthQueue as Default>::default();
        let _ = _model.size();
    }

    #[test]
    fn test_health_queue_serialization() {
        let model = <HealthQueue as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthQueue, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
