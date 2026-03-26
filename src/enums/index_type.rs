use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IndexType {
    #[serde(rename = "key")]
    #[default]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
    #[serde(rename = "spatial")]
    Spatial,
}

impl IndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            IndexType::Key => "key",
            IndexType::Fulltext => "fulltext",
            IndexType::Unique => "unique",
            IndexType::Spatial => "spatial",
        }
    }
}

impl std::fmt::Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
