//! PolicySessionDuration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Session Duration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicySessionDuration {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Session duration in seconds.
    #[serde(rename = "duration")]
    pub duration: i64,
}

impl PolicySessionDuration {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get duration
    pub fn duration(&self) -> &i64 {
        &self.duration
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_session_duration_creation() {
        let _model = <PolicySessionDuration as Default>::default();
        let _ = _model.id();
        let _ = _model.duration();
    }

    #[test]
    fn test_policy_session_duration_serialization() {
        let model = <PolicySessionDuration as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicySessionDuration, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
