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
}

impl DatabasesIndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DatabasesIndexType::Key => "key",
            DatabasesIndexType::Fulltext => "fulltext",
            DatabasesIndexType::Unique => "unique",
            DatabasesIndexType::Spatial => "spatial",
        }
    }
}

impl std::fmt::Display for DatabasesIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
