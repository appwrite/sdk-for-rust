//! DomainsList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Domains list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DomainsList {
    /// Total number of domains that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of domains.
    #[serde(rename = "domains")]
    pub domains: Vec<crate::models::Domain>,
}

fn default_total() -> i64 {
    0
}

impl DomainsList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get domains
    pub fn domains(&self) -> &Vec<crate::models::Domain> {
        &self.domains
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domains_list_creation() {
        let _model = <DomainsList as Default>::default();
        let _ = _model.total();
        let _ = _model.domains();
    }

    #[test]
    fn test_domains_list_serialization() {
        let model = <DomainsList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DomainsList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
