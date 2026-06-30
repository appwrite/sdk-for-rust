use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HealthQueueName {
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
    #[serde(rename = "v1-notifications")]
    V1Notifications,
}

impl HealthQueueName {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            HealthQueueName::V1Database => "v1-database",
            HealthQueueName::V1Deletes => "v1-deletes",
            HealthQueueName::V1Audits => "v1-audits",
            HealthQueueName::V1Mails => "v1-mails",
            HealthQueueName::V1Functions => "v1-functions",
            HealthQueueName::V1StatsResources => "v1-stats-resources",
            HealthQueueName::V1StatsUsage => "v1-stats-usage",
            HealthQueueName::V1Webhooks => "v1-webhooks",
            HealthQueueName::V1Certificates => "v1-certificates",
            HealthQueueName::V1Builds => "v1-builds",
            HealthQueueName::V1Screenshots => "v1-screenshots",
            HealthQueueName::V1Messaging => "v1-messaging",
            HealthQueueName::V1Migrations => "v1-migrations",
            HealthQueueName::V1Notifications => "v1-notifications",
        }
    }
}

impl std::fmt::Display for HealthQueueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
