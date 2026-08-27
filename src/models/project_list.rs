//! ProjectList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Projects List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProjectList {
    /// Total number of projects that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of projects.
    #[serde(rename = "projects")]
    pub projects: Vec<crate::models::Project>,
}

impl ProjectList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get projects
    pub fn projects(&self) -> &Vec<crate::models::Project> {
        &self.projects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_list_creation() {
        let _model = <ProjectList as Default>::default();
        let _ = _model.total();
        let _ = _model.projects();
    }

    #[test]
    fn test_project_list_serialization() {
        let model = <ProjectList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProjectList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
