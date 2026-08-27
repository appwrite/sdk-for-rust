use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DocumentsDBIndexType {
    #[serde(rename = "key")]
    #[default]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
}

impl DocumentsDBIndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DocumentsDBIndexType::Key => "key",
            DocumentsDBIndexType::Fulltext => "fulltext",
            DocumentsDBIndexType::Unique => "unique",
        }
    }
}

impl std::fmt::Display for DocumentsDBIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
