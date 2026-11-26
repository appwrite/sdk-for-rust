use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AlertMetric {
    #[serde(rename = "network.requests")]
    #[default]
    NetworkRequests,
    #[serde(rename = "network.inbound")]
    NetworkInbound,
    #[serde(rename = "network.outbound")]
    NetworkOutbound,
    #[serde(rename = "executions")]
    Executions,
    #[serde(rename = "executions.mbSeconds")]
    ExecutionsMbSeconds,
    #[serde(rename = "realtime.connections")]
    RealtimeConnections,
    #[serde(rename = "webhooks.events.sent")]
    WebhooksEventsSent,
    #[serde(rename = "webhooks.events.failed")]
    WebhooksEventsFailed,
    #[serde(rename = "users.mau")]
    UsersMau,
    #[serde(rename = "auth.method.phone")]
    AuthMethodPhone,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "databases.operations.reads")]
    DatabasesOperationsReads,
    #[serde(rename = "databases.operations.writes")]
    DatabasesOperationsWrites,
    #[serde(rename = "databases.storage")]
    DatabasesStorage,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "files.storage")]
    FilesStorage,
    #[serde(rename = "messages.sent")]
    MessagesSent,
    #[serde(rename = "messages.sms.sent")]
    MessagesSmsSent,
    #[serde(rename = "messages.failed")]
    MessagesFailed,
    #[serde(rename = "avatars.screenshotsGenerated")]
    AvatarsScreenshotsGenerated,
    #[serde(rename = "dedicatedDatabases.cpu")]
    DedicatedDatabasesCpu,
    #[serde(rename = "dedicatedDatabases.memory")]
    DedicatedDatabasesMemory,
    #[serde(rename = "dedicatedDatabases.storage")]
    DedicatedDatabasesStorage,
    #[serde(rename = "dedicatedDatabases.connections")]
    DedicatedDatabasesConnections,
    #[serde(rename = "dedicatedDatabases.qps")]
    DedicatedDatabasesQps,
    #[serde(rename = "dedicatedDatabases.iopsRead")]
    DedicatedDatabasesIopsRead,
    #[serde(rename = "dedicatedDatabases.iopsWrite")]
    DedicatedDatabasesIopsWrite,
    #[serde(rename = "analytics.events")]
    AnalyticsEvents,
    #[serde(rename = "analytics.pageviews")]
    AnalyticsPageviews,
    #[serde(rename = "billing.budgetUsed")]
    BillingBudgetUsed,
}

impl AlertMetric {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AlertMetric::NetworkRequests => "network.requests",
            AlertMetric::NetworkInbound => "network.inbound",
            AlertMetric::NetworkOutbound => "network.outbound",
            AlertMetric::Executions => "executions",
            AlertMetric::ExecutionsMbSeconds => "executions.mbSeconds",
            AlertMetric::RealtimeConnections => "realtime.connections",
            AlertMetric::WebhooksEventsSent => "webhooks.events.sent",
            AlertMetric::WebhooksEventsFailed => "webhooks.events.failed",
            AlertMetric::UsersMau => "users.mau",
            AlertMetric::AuthMethodPhone => "auth.method.phone",
            AlertMetric::Users => "users",
            AlertMetric::DatabasesOperationsReads => "databases.operations.reads",
            AlertMetric::DatabasesOperationsWrites => "databases.operations.writes",
            AlertMetric::DatabasesStorage => "databases.storage",
            AlertMetric::Storage => "storage",
            AlertMetric::FilesStorage => "files.storage",
            AlertMetric::MessagesSent => "messages.sent",
            AlertMetric::MessagesSmsSent => "messages.sms.sent",
            AlertMetric::MessagesFailed => "messages.failed",
            AlertMetric::AvatarsScreenshotsGenerated => "avatars.screenshotsGenerated",
            AlertMetric::DedicatedDatabasesCpu => "dedicatedDatabases.cpu",
            AlertMetric::DedicatedDatabasesMemory => "dedicatedDatabases.memory",
            AlertMetric::DedicatedDatabasesStorage => "dedicatedDatabases.storage",
            AlertMetric::DedicatedDatabasesConnections => "dedicatedDatabases.connections",
            AlertMetric::DedicatedDatabasesQps => "dedicatedDatabases.qps",
            AlertMetric::DedicatedDatabasesIopsRead => "dedicatedDatabases.iopsRead",
            AlertMetric::DedicatedDatabasesIopsWrite => "dedicatedDatabases.iopsWrite",
            AlertMetric::AnalyticsEvents => "analytics.events",
            AlertMetric::AnalyticsPageviews => "analytics.pageviews",
            AlertMetric::BillingBudgetUsed => "billing.budgetUsed",
        }
    }
}

impl std::fmt::Display for AlertMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
