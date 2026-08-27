use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VectorsDBIndexType {
    #[serde(rename = "hnsw_euclidean")]
    #[default]
    HnswEuclidean,
    #[serde(rename = "hnsw_dot")]
    HnswDot,
    #[serde(rename = "hnsw_cosine")]
    HnswCosine,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "unique")]
    Unique,
}

impl VectorsDBIndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VectorsDBIndexType::HnswEuclidean => "hnsw_euclidean",
            VectorsDBIndexType::HnswDot => "hnsw_dot",
            VectorsDBIndexType::HnswCosine => "hnsw_cosine",
            VectorsDBIndexType::Object => "object",
            VectorsDBIndexType::Key => "key",
            VectorsDBIndexType::Unique => "unique",
        }
    }
}

impl std::fmt::Display for VectorsDBIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
