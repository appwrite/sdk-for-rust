//! BucketList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Buckets List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BucketList {
    /// Total number of buckets that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of buckets.
    #[serde(rename = "buckets")]
    pub buckets: Vec<crate::models::Bucket>,
}

impl BucketList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get buckets
    pub fn buckets(&self) -> &Vec<crate::models::Bucket> {
        &self.buckets
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_list_creation() {
        let _model = <BucketList as Default>::default();
        let _ = _model.total();
        let _ = _model.buckets();
    }

    #[test]
    fn test_bucket_list_serialization() {
        let model = <BucketList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BucketList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
