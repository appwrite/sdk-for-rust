//! AlgoBcrypt model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoBcrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoBcrypt {
    /// Algo type.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
}

fn default_type() -> String {
    "bcrypt".to_string()
}

impl AlgoBcrypt {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_bcrypt_creation() {
        let _model = <AlgoBcrypt as Default>::default();
        let _ = _model.r#type();
    }

    #[test]
    fn test_algo_bcrypt_serialization() {
        let model = <AlgoBcrypt as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoBcrypt, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
