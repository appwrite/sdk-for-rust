//! Oauth2OrganizationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 accessible organizations list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2OrganizationList {
    /// Total number of organizations that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of organizations.
    #[serde(rename = "organizations")]
    pub organizations: Vec<crate::models::Oauth2Organization>,
}

impl Oauth2OrganizationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get organizations
    pub fn organizations(&self) -> &Vec<crate::models::Oauth2Organization> {
        &self.organizations
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_organization_list_creation() {
        let _model = <Oauth2OrganizationList as Default>::default();
        let _ = _model.total();
        let _ = _model.organizations();
    }

    #[test]
    fn test_oauth2_organization_list_serialization() {
        let model = <Oauth2OrganizationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2OrganizationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
