//! Membership model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Membership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Membership {
    /// Membership ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Membership creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Membership update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
    /// User name. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "userName")]
    #[serde(default = "default_user_name")]
    pub user_name: String,
    /// User email address. Hide this attribute by toggling membership privacy in
    /// the Console.
    #[serde(rename = "userEmail")]
    #[serde(default = "default_user_email")]
    pub user_email: String,
    /// User phone number. Hide this attribute by toggling membership privacy in
    /// the Console.
    #[serde(rename = "userPhone")]
    #[serde(default = "default_user_phone")]
    pub user_phone: String,
    /// Team ID.
    #[serde(rename = "teamId")]
    #[serde(default = "default_team_id")]
    pub team_id: String,
    /// Team name.
    #[serde(rename = "teamName")]
    #[serde(default = "default_team_name")]
    pub team_name: String,
    /// Date, the user has been invited to join the team in ISO 8601 format.
    #[serde(rename = "invited")]
    #[serde(default = "default_invited")]
    pub invited: String,
    /// Date, the user has accepted the invitation to join the team in ISO 8601
    /// format.
    #[serde(rename = "joined")]
    #[serde(default = "default_joined")]
    pub joined: String,
    /// User confirmation status, true if the user has joined the team or false
    /// otherwise.
    #[serde(rename = "confirm")]
    #[serde(default = "default_confirm")]
    pub confirm: bool,
    /// Multi factor authentication status, true if the user has MFA enabled or
    /// false otherwise. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "mfa")]
    #[serde(default = "default_mfa")]
    pub mfa: bool,
    /// Most recent access date in ISO 8601 format. Show this attribute by toggling
    /// membership privacy in the Console.
    #[serde(rename = "userAccessedAt")]
    #[serde(default = "default_user_accessed_at")]
    pub user_accessed_at: String,
    /// User list of roles
    #[serde(rename = "roles")]
    #[serde(default = "default_roles")]
    pub roles: Vec<String>,
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

fn default_user_id() -> String {
    "".to_string()
}

fn default_user_name() -> String {
    "".to_string()
}

fn default_user_email() -> String {
    "".to_string()
}

fn default_user_phone() -> String {
    "".to_string()
}

fn default_team_id() -> String {
    "".to_string()
}

fn default_team_name() -> String {
    "".to_string()
}

fn default_invited() -> String {
    "".to_string()
}

fn default_joined() -> String {
    "".to_string()
}

fn default_confirm() -> bool {
    false
}

fn default_mfa() -> bool {
    false
}

fn default_user_accessed_at() -> String {
    "".to_string()
}

fn default_roles() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl Membership {
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

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get user_name
    pub fn user_name(&self) -> &String {
        &self.user_name
    }

    /// Get user_email
    pub fn user_email(&self) -> &String {
        &self.user_email
    }

    /// Get user_phone
    pub fn user_phone(&self) -> &String {
        &self.user_phone
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get team_name
    pub fn team_name(&self) -> &String {
        &self.team_name
    }

    /// Get invited
    pub fn invited(&self) -> &String {
        &self.invited
    }

    /// Get joined
    pub fn joined(&self) -> &String {
        &self.joined
    }

    /// Get confirm
    pub fn confirm(&self) -> &bool {
        &self.confirm
    }

    /// Get mfa
    pub fn mfa(&self) -> &bool {
        &self.mfa
    }

    /// Get user_accessed_at
    pub fn user_accessed_at(&self) -> &String {
        &self.user_accessed_at
    }

    /// Get roles
    pub fn roles(&self) -> &Vec<String> {
        &self.roles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membership_creation() {
        let _model = <Membership as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.user_id();
        let _ = _model.user_name();
        let _ = _model.user_email();
        let _ = _model.user_phone();
        let _ = _model.team_id();
        let _ = _model.team_name();
        let _ = _model.invited();
        let _ = _model.joined();
        let _ = _model.confirm();
        let _ = _model.mfa();
        let _ = _model.user_accessed_at();
        let _ = _model.roles();
    }

    #[test]
    fn test_membership_serialization() {
        let model = <Membership as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Membership, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
