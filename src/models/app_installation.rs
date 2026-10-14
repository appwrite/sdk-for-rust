//! AppInstallation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AppInstallation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppInstallation {
    /// Installation ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Installation creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Installation update time in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// ID of the installed application.
    #[serde(rename = "appId")]
    #[serde(default = "default_app_id")]
    pub app_id: String,
    /// ID of the team the application is installed on.
    #[serde(rename = "teamId")]
    #[serde(default = "default_team_id")]
    pub team_id: String,
    /// Scopes granted to the application. Snapshot of the application's
    /// installation scopes taken when the installation was created or last
    /// updated.
    #[serde(rename = "scopes")]
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    /// Authorization details granted to the application. Rich authorization
    /// request (RFC 9396) style entries; the Appwrite Console stores authorized
    /// project IDs here.
    #[serde(rename = "authorizationDetails")]
    #[serde(default = "default_authorization_details")]
    pub authorization_details: Vec<serde_json::Value>,
    /// ID of the user who created the installation.
    #[serde(rename = "createdById")]
    #[serde(default = "default_created_by_id")]
    pub created_by_id: String,
    /// Name of the user who created the installation.
    #[serde(rename = "createdByName")]
    #[serde(default = "default_created_by_name")]
    pub created_by_name: String,
    /// Time an access token was last issued for the installation in ISO 8601
    /// format. Null if never used.
    #[serde(rename = "lastAccessedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_app_id() -> String {
    "".to_string()
}

fn default_team_id() -> String {
    "".to_string()
}

fn default_scopes() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_authorization_details() -> Vec<serde_json::Value> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_created_by_id() -> String {
    "".to_string()
}

fn default_created_by_name() -> String {
    "".to_string()
}

impl AppInstallation {
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

    /// Get app_id
    pub fn app_id(&self) -> &String {
        &self.app_id
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get scopes
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    /// Get authorization_details
    pub fn authorization_details(&self) -> &Vec<serde_json::Value> {
        &self.authorization_details
    }

    /// Get created_by_id
    pub fn created_by_id(&self) -> &String {
        &self.created_by_id
    }

    /// Get created_by_name
    pub fn created_by_name(&self) -> &String {
        &self.created_by_name
    }

    /// Set last_accessed_at
    pub fn set_last_accessed_at(mut self, last_accessed_at: String) -> Self {
        self.last_accessed_at = Some(last_accessed_at);
        self
    }

    /// Get last_accessed_at
    pub fn last_accessed_at(&self) -> Option<&String> {
        self.last_accessed_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_installation_creation() {
        let _model = <AppInstallation as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.app_id();
        let _ = _model.team_id();
        let _ = _model.scopes();
        let _ = _model.authorization_details();
        let _ = _model.created_by_id();
        let _ = _model.created_by_name();
    }

    #[test]
    fn test_app_installation_serialization() {
        let model = <AppInstallation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppInstallation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
