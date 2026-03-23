//! MfaChallenge model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFA Challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaChallenge {
    /// Token ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    pub expire: String,
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
