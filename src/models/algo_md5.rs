//! AlgoMd5 model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoMD5
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoMd5 {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl AlgoMd5 {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_md5_creation() {
        let _model = <AlgoMd5 as Default>::default();
        let _ = _model.r#type();
    }

    #[test]
    fn test_algo_md5_serialization() {
        let model = <AlgoMd5 as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoMd5, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
