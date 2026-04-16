use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ServiceId {
    #[serde(rename = "account")]
    #[default]
    Account,
    #[serde(rename = "avatars")]
    Avatars,
    #[serde(rename = "databases")]
    Databases,
    #[serde(rename = "tablesdb")]
    Tablesdb,
    #[serde(rename = "locale")]
    Locale,
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "teams")]
    Teams,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "vcs")]
    Vcs,
    #[serde(rename = "sites")]
    Sites,
    #[serde(rename = "functions")]
    Functions,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "migrations")]
    Migrations,
    #[serde(rename = "messaging")]
    Messaging,
}

impl ServiceId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ServiceId::Account => "account",
            ServiceId::Avatars => "avatars",
            ServiceId::Databases => "databases",
            ServiceId::Tablesdb => "tablesdb",
            ServiceId::Locale => "locale",
            ServiceId::Health => "health",
            ServiceId::Project => "project",
            ServiceId::Storage => "storage",
            ServiceId::Teams => "teams",
            ServiceId::Users => "users",
            ServiceId::Vcs => "vcs",
            ServiceId::Sites => "sites",
            ServiceId::Functions => "functions",
            ServiceId::Proxy => "proxy",
            ServiceId::Graphql => "graphql",
            ServiceId::Migrations => "migrations",
            ServiceId::Messaging => "messaging",
        }
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
