use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NHostMigrationResource {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "collection")]
    Collection,
    #[serde(rename = "bucket")]
    Bucket,
    #[serde(rename = "file")]
    File,
}

impl NHostMigrationResource {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            NHostMigrationResource::User => "user",
            NHostMigrationResource::Database => "database",
            NHostMigrationResource::Table => "table",
            NHostMigrationResource::Column => "column",
            NHostMigrationResource::Index => "index",
            NHostMigrationResource::Row => "row",
            NHostMigrationResource::Document => "document",
            NHostMigrationResource::Attribute => "attribute",
            NHostMigrationResource::Collection => "collection",
            NHostMigrationResource::Bucket => "bucket",
            NHostMigrationResource::File => "file",
        }
    }
}

impl std::fmt::Display for NHostMigrationResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
