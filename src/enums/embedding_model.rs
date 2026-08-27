use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmbeddingModel {
    #[serde(rename = "nomic-embed-text")]
    #[default]
    NomicEmbedText,
    #[serde(rename = "all-minilm")]
    AllMinilm,
}

impl EmbeddingModel {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            EmbeddingModel::NomicEmbedText => "nomic-embed-text",
            EmbeddingModel::AllMinilm => "all-minilm",
        }
    }
}

impl std::fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
