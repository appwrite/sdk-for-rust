//! AlgoScrypt model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoScrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoScrypt {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// CPU complexity of computed hash.
    #[serde(rename = "costCpu")]
    pub cost_cpu: i64,
    /// Memory complexity of computed hash.
    #[serde(rename = "costMemory")]
    pub cost_memory: i64,
    /// Parallelization of computed hash.
    #[serde(rename = "costParallel")]
    pub cost_parallel: i64,
    /// Length used to compute hash.
    #[serde(rename = "length")]
    pub length: i64,
}

impl AlgoScrypt {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get cost_cpu
    pub fn cost_cpu(&self) -> &i64 {
        &self.cost_cpu
    }

    /// Get cost_memory
    pub fn cost_memory(&self) -> &i64 {
        &self.cost_memory
    }

    /// Get cost_parallel
    pub fn cost_parallel(&self) -> &i64 {
        &self.cost_parallel
    }

    /// Get length
    pub fn length(&self) -> &i64 {
        &self.length
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_scrypt_creation() {
        let _model = <AlgoScrypt as Default>::default();
        let _ = _model.r#type();
        let _ = _model.cost_cpu();
        let _ = _model.cost_memory();
        let _ = _model.cost_parallel();
        let _ = _model.length();
    }

    #[test]
    fn test_algo_scrypt_serialization() {
        let model = <AlgoScrypt as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoScrypt, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
