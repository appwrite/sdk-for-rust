//! PolicyPasswordPwned model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password Pwned
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordPwned {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Whether passwords are checked against known data breaches and the result
    /// recorded on the user.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Whether a sign-in with a breached password is refused until the password is
    /// reset.
    #[serde(rename = "sessions")]
    #[serde(default = "default_sessions")]
    pub sessions: bool,
    /// Whether a breached password is rejected when a user signs up or sets a new
    /// password.
    #[serde(rename = "users")]
    #[serde(default = "default_users")]
    pub users: bool,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    true
}

fn default_sessions() -> bool {
    false
}

fn default_users() -> bool {
    false
}

impl PolicyPasswordPwned {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get sessions
    pub fn sessions(&self) -> &bool {
        &self.sessions
    }

    /// Get users
    pub fn users(&self) -> &bool {
        &self.users
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_password_pwned_creation() {
        let _model = <PolicyPasswordPwned as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
        let _ = _model.sessions();
        let _ = _model.users();
    }

    #[test]
    fn test_policy_password_pwned_serialization() {
        let model = <PolicyPasswordPwned as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordPwned, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
