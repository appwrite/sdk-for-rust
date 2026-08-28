//! Embedding model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Embedding {
    /// Embedding model used to generate embeddings.
    #[serde(rename = "model")]
    pub model: String,
    /// Number of dimensions for each embedding vector.
    #[serde(rename = "dimension")]
    pub dimension: i64,
    /// Embedding vector values. If an error occurs, this will be an empty array.
    #[serde(rename = "embedding")]
    pub embedding: Vec<f64>,
    /// Error message if embedding generation fails. Empty string if no error.
    #[serde(rename = "error")]
    pub error: String,
}

impl Embedding {
    /// Get model
    pub fn model(&self) -> &String {
        &self.model
    }

    /// Get dimension
    pub fn dimension(&self) -> &i64 {
        &self.dimension
    }

    /// Get embedding
    pub fn embedding(&self) -> &Vec<f64> {
        &self.embedding
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let _model = <Embedding as Default>::default();
        let _ = _model.model();
        let _ = _model.dimension();
        let _ = _model.embedding();
        let _ = _model.error();
    }

    #[test]
    fn test_embedding_serialization() {
        let model = <Embedding as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Embedding, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
