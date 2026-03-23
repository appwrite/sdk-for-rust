//! Team model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Team {
    /// Team ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Team creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Team update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Team name.
    #[serde(rename = "name")]
    pub name: String,
    /// Total number of team members.
    #[serde(rename = "total")]
    pub total: i64,
    /// Team preferences as a key-value object
    #[serde(rename = "prefs")]
    pub prefs: crate::models::Preferences,
}

impl Team {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get prefs
    pub fn prefs(&self) -> &crate::models::Preferences {
        &self.prefs
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_creation() {
        let _model = <Team as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.total();
        let _ = _model.prefs();
    }

    #[test]
    fn test_team_serialization() {
        let model = <Team as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Team, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
