//! Passkey model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Passkey {
    /// Passkey ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Passkey creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Passkey update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Passkey name.
    #[serde(rename = "name")]
    pub name: String,
    /// Most recent sign-in with this passkey in ISO 8601 format. Empty until the
    /// passkey is used to sign in.
    #[serde(rename = "accessedAt")]
    pub accessed_at: String,
}

impl Passkey {
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

    /// Get accessed_at
    pub fn accessed_at(&self) -> &String {
        &self.accessed_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_creation() {
        let _model = <Passkey as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.accessed_at();
    }

    #[test]
    fn test_passkey_serialization() {
        let model = <Passkey as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Passkey, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
