//! EmbeddingList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Embedding list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EmbeddingList {
    /// Total number of embeddings that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of embeddings.
    #[serde(rename = "embeddings")]
    pub embeddings: Vec<crate::models::Embedding>,
}

impl EmbeddingList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get embeddings
    pub fn embeddings(&self) -> &Vec<crate::models::Embedding> {
        &self.embeddings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_list_creation() {
        let _model = <EmbeddingList as Default>::default();
        let _ = _model.total();
        let _ = _model.embeddings();
    }

    #[test]
    fn test_embedding_list_serialization() {
        let model = <EmbeddingList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EmbeddingList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
