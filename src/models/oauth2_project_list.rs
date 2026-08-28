//! Oauth2ProjectList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 accessible projects list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2ProjectList {
    /// Total number of projects that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of projects.
    #[serde(rename = "projects")]
    pub projects: Vec<crate::models::Oauth2Project>,
}

impl Oauth2ProjectList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get projects
    pub fn projects(&self) -> &Vec<crate::models::Oauth2Project> {
        &self.projects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_project_list_creation() {
        let _model = <Oauth2ProjectList as Default>::default();
        let _ = _model.total();
        let _ = _model.projects();
    }

    #[test]
    fn test_oauth2_project_list_serialization() {
        let model = <Oauth2ProjectList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2ProjectList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
