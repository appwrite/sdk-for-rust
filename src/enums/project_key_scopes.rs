use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectKeyScopes {
    #[serde(rename = "project.read")]
    #[default]
    ProjectRead,
    #[serde(rename = "project.write")]
    ProjectWrite,
    #[serde(rename = "usage.read")]
    UsageRead,
    #[serde(rename = "keys.read")]
    KeysRead,
    #[serde(rename = "keys.write")]
    KeysWrite,
    #[serde(rename = "platforms.read")]
    PlatformsRead,
    #[serde(rename = "platforms.write")]
    PlatformsWrite,
    #[serde(rename = "mocks.read")]
    MocksRead,
    #[serde(rename = "mocks.write")]
    MocksWrite,
    #[serde(rename = "policies.read")]
    PoliciesRead,
    #[serde(rename = "policies.write")]
    PoliciesWrite,
    #[serde(rename = "project.policies.read")]
    ProjectPoliciesRead,
    #[serde(rename = "project.policies.write")]
    ProjectPoliciesWrite,
    #[serde(rename = "project.oauth2.read")]
    ProjectOauth2Read,
    #[serde(rename = "project.oauth2.write")]
    ProjectOauth2Write,
    #[serde(rename = "templates.read")]
    TemplatesRead,
    #[serde(rename = "templates.write")]
    TemplatesWrite,
    #[serde(rename = "stages.read")]
    StagesRead,
    #[serde(rename = "stages.write")]
    StagesWrite,
    #[serde(rename = "users.read")]
    UsersRead,
    #[serde(rename = "users.write")]
    UsersWrite,
    #[serde(rename = "sessions.read")]
    SessionsRead,
    #[serde(rename = "sessions.write")]
    SessionsWrite,
    #[serde(rename = "teams.read")]
    TeamsRead,
    #[serde(rename = "teams.write")]
    TeamsWrite,
    #[serde(rename = "databases.read")]
    DatabasesRead,
    #[serde(rename = "databases.write")]
    DatabasesWrite,
    #[serde(rename = "tables.read")]
    TablesRead,
    #[serde(rename = "tables.write")]
    TablesWrite,
    #[serde(rename = "columns.read")]
    ColumnsRead,
    #[serde(rename = "columns.write")]
    ColumnsWrite,
    #[serde(rename = "indexes.read")]
    IndexesRead,
    #[serde(rename = "indexes.write")]
    IndexesWrite,
    #[serde(rename = "rows.read")]
    RowsRead,
    #[serde(rename = "rows.write")]
    RowsWrite,
    #[serde(rename = "embeddings.write")]
    EmbeddingsWrite,
    #[serde(rename = "collections.read")]
    CollectionsRead,
    #[serde(rename = "collections.write")]
    CollectionsWrite,
    #[serde(rename = "attributes.read")]
    AttributesRead,
    #[serde(rename = "attributes.write")]
    AttributesWrite,
    #[serde(rename = "documents.read")]
    DocumentsRead,
    #[serde(rename = "documents.write")]
    DocumentsWrite,
    #[serde(rename = "documentsdb.read")]
    DocumentsdbRead,
    #[serde(rename = "documentsdb.write")]
    DocumentsdbWrite,
    #[serde(rename = "documentsdb.collections.read")]
    DocumentsdbCollectionsRead,
    #[serde(rename = "documentsdb.collections.write")]
    DocumentsdbCollectionsWrite,
    #[serde(rename = "documentsdb.documents.read")]
    DocumentsdbDocumentsRead,
    #[serde(rename = "documentsdb.documents.write")]
    DocumentsdbDocumentsWrite,
    #[serde(rename = "vectorsdb.read")]
    VectorsdbRead,
    #[serde(rename = "vectorsdb.write")]
    VectorsdbWrite,
    #[serde(rename = "vectorsdb.collections.read")]
    VectorsdbCollectionsRead,
    #[serde(rename = "vectorsdb.collections.write")]
    VectorsdbCollectionsWrite,
    #[serde(rename = "vectorsdb.documents.read")]
    VectorsdbDocumentsRead,
    #[serde(rename = "vectorsdb.documents.write")]
    VectorsdbDocumentsWrite,
    #[serde(rename = "buckets.read")]
    BucketsRead,
    #[serde(rename = "buckets.write")]
    BucketsWrite,
    #[serde(rename = "files.read")]
    FilesRead,
    #[serde(rename = "files.write")]
    FilesWrite,
    #[serde(rename = "tokens.read")]
    TokensRead,
    #[serde(rename = "tokens.write")]
    TokensWrite,
    #[serde(rename = "functions.read")]
    FunctionsRead,
    #[serde(rename = "functions.write")]
    FunctionsWrite,
    #[serde(rename = "executions.read")]
    ExecutionsRead,
    #[serde(rename = "executions.write")]
    ExecutionsWrite,
    #[serde(rename = "execution.read")]
    ExecutionRead,
    #[serde(rename = "execution.write")]
    ExecutionWrite,
    #[serde(rename = "sites.read")]
    SitesRead,
    #[serde(rename = "sites.write")]
    SitesWrite,
    #[serde(rename = "log.read")]
    LogRead,
    #[serde(rename = "log.write")]
    LogWrite,
    #[serde(rename = "providers.read")]
    ProvidersRead,
    #[serde(rename = "providers.write")]
    ProvidersWrite,
    #[serde(rename = "topics.read")]
    TopicsRead,
    #[serde(rename = "topics.write")]
    TopicsWrite,
    #[serde(rename = "subscribers.read")]
    SubscribersRead,
    #[serde(rename = "subscribers.write")]
    SubscribersWrite,
    #[serde(rename = "targets.read")]
    TargetsRead,
    #[serde(rename = "targets.write")]
    TargetsWrite,
    #[serde(rename = "messages.read")]
    MessagesRead,
    #[serde(rename = "messages.write")]
    MessagesWrite,
    #[serde(rename = "rules.read")]
    RulesRead,
    #[serde(rename = "rules.write")]
    RulesWrite,
    #[serde(rename = "webhooks.read")]
    WebhooksRead,
    #[serde(rename = "webhooks.write")]
    WebhooksWrite,
    #[serde(rename = "locale.read")]
    LocaleRead,
    #[serde(rename = "avatars.read")]
    AvatarsRead,
    #[serde(rename = "health.read")]
    HealthRead,
    #[serde(rename = "assistant.read")]
    AssistantRead,
    #[serde(rename = "migrations.read")]
    MigrationsRead,
    #[serde(rename = "migrations.write")]
    MigrationsWrite,
    #[serde(rename = "schedules.read")]
    SchedulesRead,
    #[serde(rename = "schedules.write")]
    SchedulesWrite,
    #[serde(rename = "vcs.read")]
    VcsRead,
    #[serde(rename = "vcs.write")]
    VcsWrite,
    #[serde(rename = "insights.read")]
    InsightsRead,
    #[serde(rename = "insights.write")]
    InsightsWrite,
    #[serde(rename = "reports.read")]
    ReportsRead,
    #[serde(rename = "reports.write")]
    ReportsWrite,
    #[serde(rename = "presences.read")]
    PresencesRead,
    #[serde(rename = "presences.write")]
    PresencesWrite,
    #[serde(rename = "backups.policies.read")]
    BackupsPoliciesRead,
    #[serde(rename = "backups.policies.write")]
    BackupsPoliciesWrite,
    #[serde(rename = "archives.read")]
    ArchivesRead,
    #[serde(rename = "archives.write")]
    ArchivesWrite,
    #[serde(rename = "restorations.read")]
    RestorationsRead,
    #[serde(rename = "restorations.write")]
    RestorationsWrite,
    #[serde(rename = "domains.read")]
    DomainsRead,
    #[serde(rename = "domains.write")]
    DomainsWrite,
    #[serde(rename = "wafRules.read")]
    WafRulesRead,
    #[serde(rename = "wafRules.write")]
    WafRulesWrite,
    #[serde(rename = "events.read")]
    EventsRead,
    #[serde(rename = "proxy.invalidations.write")]
    ProxyInvalidationsWrite,
    #[serde(rename = "apps.read")]
    AppsRead,
    #[serde(rename = "apps.write")]
    AppsWrite,
    #[serde(rename = "oauth2.read")]
    Oauth2Read,
    #[serde(rename = "oauth2.write")]
    Oauth2Write,
    #[serde(rename = "oauth2.introspect")]
    Oauth2Introspect,
}

impl ProjectKeyScopes {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectKeyScopes::ProjectRead => "project.read",
            ProjectKeyScopes::ProjectWrite => "project.write",
            ProjectKeyScopes::UsageRead => "usage.read",
            ProjectKeyScopes::KeysRead => "keys.read",
            ProjectKeyScopes::KeysWrite => "keys.write",
            ProjectKeyScopes::PlatformsRead => "platforms.read",
            ProjectKeyScopes::PlatformsWrite => "platforms.write",
            ProjectKeyScopes::MocksRead => "mocks.read",
            ProjectKeyScopes::MocksWrite => "mocks.write",
            ProjectKeyScopes::PoliciesRead => "policies.read",
            ProjectKeyScopes::PoliciesWrite => "policies.write",
            ProjectKeyScopes::ProjectPoliciesRead => "project.policies.read",
            ProjectKeyScopes::ProjectPoliciesWrite => "project.policies.write",
            ProjectKeyScopes::ProjectOauth2Read => "project.oauth2.read",
            ProjectKeyScopes::ProjectOauth2Write => "project.oauth2.write",
            ProjectKeyScopes::TemplatesRead => "templates.read",
            ProjectKeyScopes::TemplatesWrite => "templates.write",
            ProjectKeyScopes::StagesRead => "stages.read",
            ProjectKeyScopes::StagesWrite => "stages.write",
            ProjectKeyScopes::UsersRead => "users.read",
            ProjectKeyScopes::UsersWrite => "users.write",
            ProjectKeyScopes::SessionsRead => "sessions.read",
            ProjectKeyScopes::SessionsWrite => "sessions.write",
            ProjectKeyScopes::TeamsRead => "teams.read",
            ProjectKeyScopes::TeamsWrite => "teams.write",
            ProjectKeyScopes::DatabasesRead => "databases.read",
            ProjectKeyScopes::DatabasesWrite => "databases.write",
            ProjectKeyScopes::TablesRead => "tables.read",
            ProjectKeyScopes::TablesWrite => "tables.write",
            ProjectKeyScopes::ColumnsRead => "columns.read",
            ProjectKeyScopes::ColumnsWrite => "columns.write",
            ProjectKeyScopes::IndexesRead => "indexes.read",
            ProjectKeyScopes::IndexesWrite => "indexes.write",
            ProjectKeyScopes::RowsRead => "rows.read",
            ProjectKeyScopes::RowsWrite => "rows.write",
            ProjectKeyScopes::EmbeddingsWrite => "embeddings.write",
            ProjectKeyScopes::CollectionsRead => "collections.read",
            ProjectKeyScopes::CollectionsWrite => "collections.write",
            ProjectKeyScopes::AttributesRead => "attributes.read",
            ProjectKeyScopes::AttributesWrite => "attributes.write",
            ProjectKeyScopes::DocumentsRead => "documents.read",
            ProjectKeyScopes::DocumentsWrite => "documents.write",
            ProjectKeyScopes::DocumentsdbRead => "documentsdb.read",
            ProjectKeyScopes::DocumentsdbWrite => "documentsdb.write",
            ProjectKeyScopes::DocumentsdbCollectionsRead => "documentsdb.collections.read",
            ProjectKeyScopes::DocumentsdbCollectionsWrite => "documentsdb.collections.write",
            ProjectKeyScopes::DocumentsdbDocumentsRead => "documentsdb.documents.read",
            ProjectKeyScopes::DocumentsdbDocumentsWrite => "documentsdb.documents.write",
            ProjectKeyScopes::VectorsdbRead => "vectorsdb.read",
            ProjectKeyScopes::VectorsdbWrite => "vectorsdb.write",
            ProjectKeyScopes::VectorsdbCollectionsRead => "vectorsdb.collections.read",
            ProjectKeyScopes::VectorsdbCollectionsWrite => "vectorsdb.collections.write",
            ProjectKeyScopes::VectorsdbDocumentsRead => "vectorsdb.documents.read",
            ProjectKeyScopes::VectorsdbDocumentsWrite => "vectorsdb.documents.write",
            ProjectKeyScopes::BucketsRead => "buckets.read",
            ProjectKeyScopes::BucketsWrite => "buckets.write",
            ProjectKeyScopes::FilesRead => "files.read",
            ProjectKeyScopes::FilesWrite => "files.write",
            ProjectKeyScopes::TokensRead => "tokens.read",
            ProjectKeyScopes::TokensWrite => "tokens.write",
            ProjectKeyScopes::FunctionsRead => "functions.read",
            ProjectKeyScopes::FunctionsWrite => "functions.write",
            ProjectKeyScopes::ExecutionsRead => "executions.read",
            ProjectKeyScopes::ExecutionsWrite => "executions.write",
            ProjectKeyScopes::ExecutionRead => "execution.read",
            ProjectKeyScopes::ExecutionWrite => "execution.write",
            ProjectKeyScopes::SitesRead => "sites.read",
            ProjectKeyScopes::SitesWrite => "sites.write",
            ProjectKeyScopes::LogRead => "log.read",
            ProjectKeyScopes::LogWrite => "log.write",
            ProjectKeyScopes::ProvidersRead => "providers.read",
            ProjectKeyScopes::ProvidersWrite => "providers.write",
            ProjectKeyScopes::TopicsRead => "topics.read",
            ProjectKeyScopes::TopicsWrite => "topics.write",
            ProjectKeyScopes::SubscribersRead => "subscribers.read",
            ProjectKeyScopes::SubscribersWrite => "subscribers.write",
            ProjectKeyScopes::TargetsRead => "targets.read",
            ProjectKeyScopes::TargetsWrite => "targets.write",
            ProjectKeyScopes::MessagesRead => "messages.read",
            ProjectKeyScopes::MessagesWrite => "messages.write",
            ProjectKeyScopes::RulesRead => "rules.read",
            ProjectKeyScopes::RulesWrite => "rules.write",
            ProjectKeyScopes::WebhooksRead => "webhooks.read",
            ProjectKeyScopes::WebhooksWrite => "webhooks.write",
            ProjectKeyScopes::LocaleRead => "locale.read",
            ProjectKeyScopes::AvatarsRead => "avatars.read",
            ProjectKeyScopes::HealthRead => "health.read",
            ProjectKeyScopes::AssistantRead => "assistant.read",
            ProjectKeyScopes::MigrationsRead => "migrations.read",
            ProjectKeyScopes::MigrationsWrite => "migrations.write",
            ProjectKeyScopes::SchedulesRead => "schedules.read",
            ProjectKeyScopes::SchedulesWrite => "schedules.write",
            ProjectKeyScopes::VcsRead => "vcs.read",
            ProjectKeyScopes::VcsWrite => "vcs.write",
            ProjectKeyScopes::InsightsRead => "insights.read",
            ProjectKeyScopes::InsightsWrite => "insights.write",
            ProjectKeyScopes::ReportsRead => "reports.read",
            ProjectKeyScopes::ReportsWrite => "reports.write",
            ProjectKeyScopes::PresencesRead => "presences.read",
            ProjectKeyScopes::PresencesWrite => "presences.write",
            ProjectKeyScopes::BackupsPoliciesRead => "backups.policies.read",
            ProjectKeyScopes::BackupsPoliciesWrite => "backups.policies.write",
            ProjectKeyScopes::ArchivesRead => "archives.read",
            ProjectKeyScopes::ArchivesWrite => "archives.write",
            ProjectKeyScopes::RestorationsRead => "restorations.read",
            ProjectKeyScopes::RestorationsWrite => "restorations.write",
            ProjectKeyScopes::DomainsRead => "domains.read",
            ProjectKeyScopes::DomainsWrite => "domains.write",
            ProjectKeyScopes::WafRulesRead => "wafRules.read",
            ProjectKeyScopes::WafRulesWrite => "wafRules.write",
            ProjectKeyScopes::EventsRead => "events.read",
            ProjectKeyScopes::ProxyInvalidationsWrite => "proxy.invalidations.write",
            ProjectKeyScopes::AppsRead => "apps.read",
            ProjectKeyScopes::AppsWrite => "apps.write",
            ProjectKeyScopes::Oauth2Read => "oauth2.read",
            ProjectKeyScopes::Oauth2Write => "oauth2.write",
            ProjectKeyScopes::Oauth2Introspect => "oauth2.introspect",
        }
    }
}

impl std::fmt::Display for ProjectKeyScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
