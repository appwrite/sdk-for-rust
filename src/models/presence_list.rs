//! PresenceList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Presences List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PresenceList {
    /// Total number of presences that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of presences.
    #[serde(rename = "presences")]
    pub presences: Vec<crate::models::Presence>,
}

impl PresenceList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get presences
    pub fn presences(&self) -> &Vec<crate::models::Presence> {
        &self.presences
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_list_creation() {
        let _model = <PresenceList as Default>::default();
        let _ = _model.total();
        let _ = _model.presences();
    }

    #[test]
    fn test_presence_list_serialization() {
        let model = <PresenceList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PresenceList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
