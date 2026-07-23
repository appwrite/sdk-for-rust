//! AppScopeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// App scopes list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppScopeList {
    /// Total number of scopes that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of scopes.
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::AppScope>,
}

impl AppScopeList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get scopes
    pub fn scopes(&self) -> &Vec<crate::models::AppScope> {
        &self.scopes
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_scope_list_creation() {
        let _model = <AppScopeList as Default>::default();
        let _ = _model.total();
        let _ = _model.scopes();
    }

    #[test]
    fn test_app_scope_list_serialization() {
        let model = <AppScopeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppScopeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
