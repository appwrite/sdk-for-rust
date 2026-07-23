//! AppSecret model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AppSecret
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AppSecret {
    /// Secret ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Secret creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Secret update time in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Application ID this secret belongs to.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Always empty. The application client secret is returned only once, in the
    /// response of the createSecret method.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Last few characters of the client secret, used to help identify it.
    #[serde(rename = "hint")]
    pub hint: String,
    /// ID of the user who created the secret.
    #[serde(rename = "createdById")]
    pub created_by_id: String,
    /// Name of the user who created the secret.
    #[serde(rename = "createdByName")]
    pub created_by_name: String,
    /// Time the secret was last used for authentication in ISO 8601 format. Null
    /// if never used.
    #[serde(rename = "lastAccessedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
}

impl AppSecret {
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

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get hint
    pub fn hint(&self) -> &String {
        &self.hint
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
    fn test_app_secret_creation() {
        let _model = <AppSecret as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.app_id();
        let _ = _model.secret();
        let _ = _model.hint();
        let _ = _model.created_by_id();
        let _ = _model.created_by_name();
    }

    #[test]
    fn test_app_secret_serialization() {
        let model = <AppSecret as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AppSecret, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
