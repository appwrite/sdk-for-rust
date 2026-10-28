use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FirebaseMigrationResource {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "column")]
    Column,
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

impl FirebaseMigrationResource {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            FirebaseMigrationResource::User => "user",
            FirebaseMigrationResource::Database => "database",
            FirebaseMigrationResource::Table => "table",
            FirebaseMigrationResource::Column => "column",
            FirebaseMigrationResource::Row => "row",
            FirebaseMigrationResource::Document => "document",
            FirebaseMigrationResource::Attribute => "attribute",
            FirebaseMigrationResource::Collection => "collection",
            FirebaseMigrationResource::Bucket => "bucket",
            FirebaseMigrationResource::File => "file",
        }
    }
}

impl std::fmt::Display for FirebaseMigrationResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
