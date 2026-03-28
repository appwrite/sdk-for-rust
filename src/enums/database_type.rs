use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DatabaseType {
    #[serde(rename = "legacy")]
    #[default]
    Legacy,
    #[serde(rename = "tablesdb")]
    Tablesdb,
    #[serde(rename = "documentsdb")]
    Documentsdb,
    #[serde(rename = "vectorsdb")]
    Vectorsdb,
}

impl DatabaseType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DatabaseType::Legacy => "legacy",
            DatabaseType::Tablesdb => "tablesdb",
            DatabaseType::Documentsdb => "documentsdb",
            DatabaseType::Vectorsdb => "vectorsdb",
        }
    }
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
