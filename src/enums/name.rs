use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Name {
    #[serde(rename = "v1-database")]
    #[default]
    V1Database,
    #[serde(rename = "v1-deletes")]
    V1Deletes,
    #[serde(rename = "v1-audits")]
    V1Audits,
    #[serde(rename = "v1-mails")]
    V1Mails,
    #[serde(rename = "v1-functions")]
    V1Functions,
    #[serde(rename = "v1-stats-resources")]
    V1StatsResources,
    #[serde(rename = "v1-stats-usage")]
    V1StatsUsage,
    #[serde(rename = "v1-webhooks")]
    V1Webhooks,
    #[serde(rename = "v1-certificates")]
    V1Certificates,
    #[serde(rename = "v1-builds")]
    V1Builds,
    #[serde(rename = "v1-screenshots")]
    V1Screenshots,
    #[serde(rename = "v1-messaging")]
    V1Messaging,
    #[serde(rename = "v1-migrations")]
    V1Migrations,
}

impl Name {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Name::V1Database => "v1-database",
            Name::V1Deletes => "v1-deletes",
            Name::V1Audits => "v1-audits",
            Name::V1Mails => "v1-mails",
            Name::V1Functions => "v1-functions",
            Name::V1StatsResources => "v1-stats-resources",
            Name::V1StatsUsage => "v1-stats-usage",
            Name::V1Webhooks => "v1-webhooks",
            Name::V1Certificates => "v1-certificates",
            Name::V1Builds => "v1-builds",
            Name::V1Screenshots => "v1-screenshots",
            Name::V1Messaging => "v1-messaging",
            Name::V1Migrations => "v1-migrations",
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
