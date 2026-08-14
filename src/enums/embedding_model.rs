use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmbeddingModel {
    #[serde(rename = "nomic-embed-text")]
    #[default]
    NomicEmbedText,
    #[serde(rename = "embedding-gemma")]
    EmbeddingGemma,
    #[serde(rename = "all-minilm")]
    AllMinilm,
    #[serde(rename = "bge-small")]
    BgeSmall,
}

impl EmbeddingModel {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            EmbeddingModel::NomicEmbedText => "nomic-embed-text",
            EmbeddingModel::EmbeddingGemma => "embedding-gemma",
            EmbeddingModel::AllMinilm => "all-minilm",
            EmbeddingModel::BgeSmall => "bge-small",
        }
    }
}

impl std::fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
