//! EnvironmentList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Environments list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EnvironmentList {
    /// Total number of environments that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of environments.
    #[serde(rename = "environments")]
    #[serde(default = "default_environments")]
    pub environments: Vec<crate::models::Environment>,
}

fn default_total() -> i64 {
    0
}

fn default_environments() -> Vec<crate::models::Environment> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl EnvironmentList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get environments
    pub fn environments(&self) -> &Vec<crate::models::Environment> {
        &self.environments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_list_creation() {
        let _model = <EnvironmentList as Default>::default();
        let _ = _model.total();
        let _ = _model.environments();
    }

    #[test]
    fn test_environment_list_serialization() {
        let model = <EnvironmentList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EnvironmentList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
