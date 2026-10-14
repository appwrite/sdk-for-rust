//! PolicyPasswordRotation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password Rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordRotation {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Whether expired passwords require a reset before email/password sign-in.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Maximum password age in seconds.
    #[serde(rename = "maxAge")]
    #[serde(default = "default_max_age")]
    pub max_age: i64,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    false
}

fn default_max_age() -> i64 {
    31536000
}

impl PolicyPasswordRotation {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get max_age
    pub fn max_age(&self) -> &i64 {
        &self.max_age
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_password_rotation_creation() {
        let _model = <PolicyPasswordRotation as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.max_age();
    }

    #[test]
    fn test_policy_password_rotation_serialization() {
        let model = <PolicyPasswordRotation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordRotation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
