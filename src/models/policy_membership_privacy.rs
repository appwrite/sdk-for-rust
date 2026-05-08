//! PolicyMembershipPrivacy model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Membership Privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyMembershipPrivacy {
    /// Policy ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Whether user ID is visible in memberships.
    #[serde(rename = "userId")]
    pub user_id: bool,
    /// Whether user email is visible in memberships.
    #[serde(rename = "userEmail")]
    pub user_email: bool,
    /// Whether user phone is visible in memberships.
    #[serde(rename = "userPhone")]
    pub user_phone: bool,
    /// Whether user name is visible in memberships.
    #[serde(rename = "userName")]
    pub user_name: bool,
    /// Whether user MFA status is visible in memberships.
    #[serde(rename = "userMFA")]
    pub user_mfa: bool,
}

impl PolicyMembershipPrivacy {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get user_id
    pub fn user_id(&self) -> &bool {
        &self.user_id
    }

    /// Get user_email
    pub fn user_email(&self) -> &bool {
        &self.user_email
    }

    /// Get user_phone
    pub fn user_phone(&self) -> &bool {
        &self.user_phone
    }

    /// Get user_name
    pub fn user_name(&self) -> &bool {
        &self.user_name
    }

    /// Get user_mfa
    pub fn user_mfa(&self) -> &bool {
        &self.user_mfa
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_membership_privacy_creation() {
        let _model = <PolicyMembershipPrivacy as Default>::default();
        let _ = _model.id();
        let _ = _model.user_id();
        let _ = _model.user_email();
        let _ = _model.user_phone();
        let _ = _model.user_name();
        let _ = _model.user_mfa();
    }

    #[test]
    fn test_policy_membership_privacy_serialization() {
        let model = <PolicyMembershipPrivacy as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyMembershipPrivacy, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
