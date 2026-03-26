//! SiteList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Sites List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct SiteList {
    /// Total number of sites that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of sites.
    #[serde(rename = "sites")]
    pub sites: Vec<crate::models::Site>,
}

impl SiteList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get sites
    pub fn sites(&self) -> &Vec<crate::models::Site> {
        &self.sites
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_list_creation() {
        let _model = <SiteList as Default>::default();
        let _ = _model.total();
        let _ = _model.sites();
    }

    #[test]
    fn test_site_list_serialization() {
        let model = <SiteList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<SiteList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
