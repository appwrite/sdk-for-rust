use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TablesDBIndexType {
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

impl TablesDBIndexType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            TablesDBIndexType::Key => "key",
            TablesDBIndexType::Fulltext => "fulltext",
            TablesDBIndexType::Unique => "unique",
            TablesDBIndexType::Spatial => "spatial",
        }
    }
}

impl std::fmt::Display for TablesDBIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
