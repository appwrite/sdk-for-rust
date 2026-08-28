//! InsightCTA model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// InsightCTA
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct InsightCTA {
    /// Human-readable label for the CTA, used in UI.
    #[serde(rename = "label")]
    pub label: String,
    /// Public API service (SDK namespace) the client should invoke. Must match the
    /// engine that owns the resource — for index suggestions: databases
    /// (legacy), tablesDB, documentsDB, or vectorsDB.
    #[serde(rename = "service")]
    pub service: String,
    /// Public API method on the chosen service the client should invoke when this
    /// CTA is triggered.
    #[serde(rename = "method")]
    pub method: String,
    /// Parameter map the client should pass to the service method when this CTA is
    /// triggered. Keys match the target API's parameter names (e.g.
    /// databaseId/tableId/columns for tablesDB, databaseId/collectionId/attributes
    /// for the legacy Databases API).
    #[serde(rename = "params")]
    pub params: serde_json::Value,
}

impl InsightCTA {
    /// Get label
    pub fn label(&self) -> &String {
        &self.label
    }

    /// Get service
    pub fn service(&self) -> &String {
        &self.service
    }

    /// Get method
    pub fn method(&self) -> &String {
        &self.method
    }

    /// Get params
    pub fn params(&self) -> &serde_json::Value {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insight_cta_creation() {
        let _model = <InsightCTA as Default>::default();
        let _ = _model.label();
        let _ = _model.service();
        let _ = _model.method();
        let _ = _model.params();
    }

    #[test]
    fn test_insight_cta_serialization() {
        let model = <InsightCTA as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<InsightCTA, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
