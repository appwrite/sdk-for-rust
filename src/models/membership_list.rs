//! MembershipList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Memberships List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MembershipList {
    /// Total number of memberships that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of memberships.
    #[serde(rename = "memberships")]
    pub memberships: Vec<crate::models::Membership>,
}

impl MembershipList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get memberships
    pub fn memberships(&self) -> &Vec<crate::models::Membership> {
        &self.memberships
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membership_list_creation() {
        let _model = <MembershipList as Default>::default();
        let _ = _model.total();
        let _ = _model.memberships();
    }

    #[test]
    fn test_membership_list_serialization() {
        let model = <MembershipList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MembershipList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
