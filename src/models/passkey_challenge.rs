//! PasskeyChallenge model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Passkey Challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PasskeyChallenge {
    /// Challenge ID. When signing in, pass it back with the credential to receive
    /// a token.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Challenge creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// ID of the passkey being registered. Empty for sign-in challenges.
    #[serde(rename = "passkeyId")]
    #[serde(default = "default_passkey_id")]
    pub passkey_id: String,
    /// Challenge expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
    /// WebAuthn options in JSON form. Pass them to
    /// `PublicKeyCredential.parseCreationOptionsFromJSON()` when registering, or
    /// `PublicKeyCredential.parseRequestOptionsFromJSON()` when signing in.
    #[serde(rename = "publicKey")]
    #[serde(default = "default_public_key")]
    pub public_key: serde_json::Value,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_passkey_id() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

fn default_public_key() -> serde_json::Value {
    serde_json::json!({})
}

impl PasskeyChallenge {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get passkey_id
    pub fn passkey_id(&self) -> &String {
        &self.passkey_id
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get public_key
    pub fn public_key(&self) -> &serde_json::Value {
        &self.public_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_challenge_creation() {
        let _model = <PasskeyChallenge as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.passkey_id();
        let _ = _model.expire();
        let _ = _model.public_key();
    }

    #[test]
    fn test_passkey_challenge_serialization() {
        let model = <PasskeyChallenge as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PasskeyChallenge, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
