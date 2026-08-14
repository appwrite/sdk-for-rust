//! MfaChallengeSecret model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// MFA Challenge Secret
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MfaChallengeSecret {
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
    /// Challenge code to be delivered to the end user through a custom channel.
    #[serde(rename = "code")]
    pub code: String,
}

impl MfaChallengeSecret {
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

    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfa_challenge_secret_creation() {
        let _model = <MfaChallengeSecret as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.user_id();
        let _ = _model.expire();
        let _ = _model.code();
    }

    #[test]
    fn test_mfa_challenge_secret_serialization() {
        let model = <MfaChallengeSecret as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MfaChallengeSecret, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
