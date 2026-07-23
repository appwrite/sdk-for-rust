//! DatabaseStatusConnections model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Connections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatusConnections {
    /// Current number of active connections.
    #[serde(rename = "current")]
    pub current: i64,
    /// Maximum allowed connections.
    #[serde(rename = "max")]
    pub max: i64,
}

impl DatabaseStatusConnections {
    /// Get current
    pub fn current(&self) -> &i64 {
        &self.current
    }

    /// Get max
    pub fn max(&self) -> &i64 {
        &self.max
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_status_connections_creation() {
        let _model = <DatabaseStatusConnections as Default>::default();
        let _ = _model.current();
        let _ = _model.max();
    }

    #[test]
    fn test_database_status_connections_serialization() {
        let model = <DatabaseStatusConnections as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseStatusConnections, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
