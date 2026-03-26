//! AlgoSha model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoSHA
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoSha {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl AlgoSha {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_sha_creation() {
        let _model = <AlgoSha as Default>::default();
        let _ = _model.r#type();
    }

    #[test]
    fn test_algo_sha_serialization() {
        let model = <AlgoSha as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoSha, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
