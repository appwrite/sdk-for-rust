//! UserList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Users List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UserList {
    /// Total number of users that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of users.
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
}

impl UserList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get users
    pub fn users(&self) -> &Vec<crate::models::User> {
        &self.users
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_list_creation() {
        let _model = <UserList as Default>::default();
        let _ = _model.total();
        let _ = _model.users();
    }

    #[test]
    fn test_user_list_serialization() {
        let model = <UserList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UserList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
