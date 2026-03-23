//! ContinentList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Continents List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ContinentList {
    /// Total number of continents that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of continents.
    #[serde(rename = "continents")]
    pub continents: Vec<crate::models::Continent>,
}

impl ContinentList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get continents
    pub fn continents(&self) -> &Vec<crate::models::Continent> {
        &self.continents
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continent_list_creation() {
        let _model = <ContinentList as Default>::default();
        let _ = _model.total();
        let _ = _model.continents();
    }

    #[test]
    fn test_continent_list_serialization() {
        let model = <ContinentList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ContinentList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
