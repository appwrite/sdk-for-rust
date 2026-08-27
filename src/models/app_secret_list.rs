//! AppSecretList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// App secrets list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppSecretList {
    /// Total number of secrets that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of secrets.
    #[serde(rename = "secrets")]
    pub secrets: Vec<crate::models::AppSecret>,
}

impl AppSecretList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get secrets
    pub fn secrets(&self) -> &Vec<crate::models::AppSecret> {
        &self.secrets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_secret_list_creation() {
        let _model = <AppSecretList as Default>::default();
        let _ = _model.total();
        let _ = _model.secrets();
    }

    #[test]
    fn test_app_secret_list_serialization() {
        let model = <AppSecretList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppSecretList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
