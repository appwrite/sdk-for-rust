//! TeamList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Teams List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TeamList {
    /// Total number of teams that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of teams.
    #[serde(rename = "teams")]
    pub teams: Vec<crate::models::Team>,
}

impl TeamList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get teams
    pub fn teams(&self) -> &Vec<crate::models::Team> {
        &self.teams
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_list_creation() {
        let _model = <TeamList as Default>::default();
        let _ = _model.total();
        let _ = _model.teams();
    }

    #[test]
    fn test_team_list_serialization() {
        let model = <TeamList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<TeamList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
