use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SupabaseMigrationResource {
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

impl SupabaseMigrationResource {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            SupabaseMigrationResource::User => "user",
            SupabaseMigrationResource::Database => "database",
            SupabaseMigrationResource::Table => "table",
            SupabaseMigrationResource::Column => "column",
            SupabaseMigrationResource::Index => "index",
            SupabaseMigrationResource::Row => "row",
            SupabaseMigrationResource::Document => "document",
            SupabaseMigrationResource::Attribute => "attribute",
            SupabaseMigrationResource::Collection => "collection",
            SupabaseMigrationResource::Bucket => "bucket",
            SupabaseMigrationResource::File => "file",
        }
    }
}

impl std::fmt::Display for SupabaseMigrationResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
