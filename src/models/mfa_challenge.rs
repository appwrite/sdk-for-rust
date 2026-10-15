//! MfaChallenge model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFA Challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaChallenge {
    /// Token ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_user_id() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

impl MfaChallenge {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfa_challenge_creation() {
        let _model = <MfaChallenge as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.user_id();
        let _ = _model.expire();
    }

    #[test]
    fn test_mfa_challenge_serialization() {
        let model = <MfaChallenge as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MfaChallenge, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
