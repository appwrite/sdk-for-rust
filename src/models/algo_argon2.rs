//! AlgoArgon2 model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoArgon2
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoArgon2 {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Memory used to compute hash.
    #[serde(rename = "memoryCost")]
    pub memory_cost: i64,
    /// Amount of time consumed to compute hash
    #[serde(rename = "timeCost")]
    pub time_cost: i64,
    /// Number of threads used to compute hash.
    #[serde(rename = "threads")]
    pub threads: i64,
}

impl AlgoArgon2 {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get memory_cost
    pub fn memory_cost(&self) -> &i64 {
        &self.memory_cost
    }

    /// Get time_cost
    pub fn time_cost(&self) -> &i64 {
        &self.time_cost
    }

    /// Get threads
    pub fn threads(&self) -> &i64 {
        &self.threads
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_argon2_creation() {
        let _model = <AlgoArgon2 as Default>::default();
        let _ = _model.r#type();
        let _ = _model.memory_cost();
        let _ = _model.time_cost();
        let _ = _model.threads();
    }

    #[test]
    fn test_algo_argon2_serialization() {
        let model = <AlgoArgon2 as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoArgon2, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
