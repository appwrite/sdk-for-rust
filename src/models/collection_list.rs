//! CollectionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Collections List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct CollectionList {
    /// Total number of collections that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of collections.
    #[serde(rename = "collections")]
    pub collections: Vec<crate::models::Collection>,
}

impl CollectionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get collections
    pub fn collections(&self) -> &Vec<crate::models::Collection> {
        &self.collections
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_list_creation() {
        let _model = <CollectionList as Default>::default();
        let _ = _model.total();
        let _ = _model.collections();
    }

    #[test]
    fn test_collection_list_serialization() {
        let model = <CollectionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<CollectionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
