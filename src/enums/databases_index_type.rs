use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DatabasesIndexType {
    #[serde(rename = "key")]
    #[default]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
    #[serde(rename = "spatial")]
    Spatial,
    #[serde(rename = "trigram")]
    Trigram,
}

impl DatabasesIndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DatabasesIndexType::Key => "key",
            DatabasesIndexType::Fulltext => "fulltext",
            DatabasesIndexType::Unique => "unique",
            DatabasesIndexType::Spatial => "spatial",
            DatabasesIndexType::Trigram => "trigram",
        }
    }
}

impl std::fmt::Display for DatabasesIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
