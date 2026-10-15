//! Oauth2Organization model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Organization {
    /// Organization ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
}

fn default_id() -> String {
    "".to_string()
}

impl Oauth2Organization {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_organization_creation() {
        let _model = <Oauth2Organization as Default>::default();
        let _ = _model.id();
    }

    #[test]
    fn test_oauth2_organization_serialization() {
        let model = <Oauth2Organization as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Organization, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
