//! IdentityList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Identities List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct IdentityList {
    /// Total number of identities that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of identities.
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::Identity>,
}

impl IdentityList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get identities
    pub fn identities(&self) -> &Vec<crate::models::Identity> {
        &self.identities
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_list_creation() {
        let _model = <IdentityList as Default>::default();
        let _ = _model.total();
        let _ = _model.identities();
    }

    #[test]
    fn test_identity_list_serialization() {
        let model = <IdentityList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<IdentityList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
