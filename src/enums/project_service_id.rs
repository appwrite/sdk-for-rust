use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectServiceId {
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
    #[serde(rename = "advisor")]
    Advisor,
    #[serde(rename = "oauth2")]
    Oauth2,
}

impl ProjectServiceId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectServiceId::Account => "account",
            ProjectServiceId::Avatars => "avatars",
            ProjectServiceId::Databases => "databases",
            ProjectServiceId::Tablesdb => "tablesdb",
            ProjectServiceId::Locale => "locale",
            ProjectServiceId::Health => "health",
            ProjectServiceId::Project => "project",
            ProjectServiceId::Storage => "storage",
            ProjectServiceId::Teams => "teams",
            ProjectServiceId::Users => "users",
            ProjectServiceId::Vcs => "vcs",
            ProjectServiceId::Sites => "sites",
            ProjectServiceId::Functions => "functions",
            ProjectServiceId::Proxy => "proxy",
            ProjectServiceId::Graphql => "graphql",
            ProjectServiceId::Migrations => "migrations",
            ProjectServiceId::Messaging => "messaging",
            ProjectServiceId::Advisor => "advisor",
            ProjectServiceId::Oauth2 => "oauth2",
        }
    }
}

impl std::fmt::Display for ProjectServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
