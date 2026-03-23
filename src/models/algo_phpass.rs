//! AlgoPhpass model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoPHPass
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoPhpass {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl AlgoPhpass {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_phpass_creation() {
        let _model = <AlgoPhpass as Default>::default();
        let _ = _model.r#type();
    }

    #[test]
    fn test_algo_phpass_serialization() {
        let model = <AlgoPhpass as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoPhpass, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
