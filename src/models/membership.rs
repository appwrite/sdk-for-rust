//! Membership model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Membership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Membership {
    /// Membership ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Membership creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Membership update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// User name. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "userName")]
    pub user_name: String,
    /// User email address. Hide this attribute by toggling membership privacy in
    /// the Console.
    #[serde(rename = "userEmail")]
    pub user_email: String,
    /// Team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Team name.
    #[serde(rename = "teamName")]
    pub team_name: String,
    /// Date, the user has been invited to join the team in ISO 8601 format.
    #[serde(rename = "invited")]
    pub invited: String,
    /// Date, the user has accepted the invitation to join the team in ISO 8601
    /// format.
    #[serde(rename = "joined")]
    pub joined: String,
    /// User confirmation status, true if the user has joined the team or false
    /// otherwise.
    #[serde(rename = "confirm")]
    pub confirm: bool,
    /// Multi factor authentication status, true if the user has MFA enabled or
    /// false otherwise. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "mfa")]
    pub mfa: bool,
    /// User list of roles
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
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
        let _ = _model.team_id();
        let _ = _model.team_name();
        let _ = _model.invited();
        let _ = _model.joined();
        let _ = _model.confirm();
        let _ = _model.mfa();
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
