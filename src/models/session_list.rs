//! SessionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Sessions List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SessionList {
    /// Total number of sessions that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of sessions.
    #[serde(rename = "sessions")]
    pub sessions: Vec<crate::models::Session>,
}

impl SessionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get sessions
    pub fn sessions(&self) -> &Vec<crate::models::Session> {
        &self.sessions
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_list_creation() {
        let _model = <SessionList as Default>::default();
        let _ = _model.total();
        let _ = _model.sessions();
    }

    #[test]
    fn test_session_list_serialization() {
        let model = <SessionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<SessionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
