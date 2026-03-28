use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Scopes {
    #[serde(rename = "sessions.write")]
    #[default]
    SessionsWrite,
    #[serde(rename = "users.read")]
    UsersRead,
    #[serde(rename = "users.write")]
    UsersWrite,
    #[serde(rename = "teams.read")]
    TeamsRead,
    #[serde(rename = "teams.write")]
    TeamsWrite,
    #[serde(rename = "databases.read")]
    DatabasesRead,
    #[serde(rename = "databases.write")]
    DatabasesWrite,
    #[serde(rename = "collections.read")]
    CollectionsRead,
    #[serde(rename = "collections.write")]
    CollectionsWrite,
    #[serde(rename = "tables.read")]
    TablesRead,
    #[serde(rename = "tables.write")]
    TablesWrite,
    #[serde(rename = "attributes.read")]
    AttributesRead,
    #[serde(rename = "attributes.write")]
    AttributesWrite,
    #[serde(rename = "columns.read")]
    ColumnsRead,
    #[serde(rename = "columns.write")]
    ColumnsWrite,
    #[serde(rename = "indexes.read")]
    IndexesRead,
    #[serde(rename = "indexes.write")]
    IndexesWrite,
    #[serde(rename = "documents.read")]
    DocumentsRead,
    #[serde(rename = "documents.write")]
    DocumentsWrite,
    #[serde(rename = "rows.read")]
    RowsRead,
    #[serde(rename = "rows.write")]
    RowsWrite,
    #[serde(rename = "files.read")]
    FilesRead,
    #[serde(rename = "files.write")]
    FilesWrite,
    #[serde(rename = "buckets.read")]
    BucketsRead,
    #[serde(rename = "buckets.write")]
    BucketsWrite,
    #[serde(rename = "functions.read")]
    FunctionsRead,
    #[serde(rename = "functions.write")]
    FunctionsWrite,
    #[serde(rename = "sites.read")]
    SitesRead,
    #[serde(rename = "sites.write")]
    SitesWrite,
    #[serde(rename = "log.read")]
    LogRead,
    #[serde(rename = "log.write")]
    LogWrite,
    #[serde(rename = "execution.read")]
    ExecutionRead,
    #[serde(rename = "execution.write")]
    ExecutionWrite,
    #[serde(rename = "locale.read")]
    LocaleRead,
    #[serde(rename = "avatars.read")]
    AvatarsRead,
    #[serde(rename = "health.read")]
    HealthRead,
    #[serde(rename = "providers.read")]
    ProvidersRead,
    #[serde(rename = "providers.write")]
    ProvidersWrite,
    #[serde(rename = "messages.read")]
    MessagesRead,
    #[serde(rename = "messages.write")]
    MessagesWrite,
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
    #[serde(rename = "rules.read")]
    RulesRead,
    #[serde(rename = "rules.write")]
    RulesWrite,
    #[serde(rename = "schedules.read")]
    SchedulesRead,
    #[serde(rename = "schedules.write")]
    SchedulesWrite,
    #[serde(rename = "migrations.read")]
    MigrationsRead,
    #[serde(rename = "migrations.write")]
    MigrationsWrite,
    #[serde(rename = "vcs.read")]
    VcsRead,
    #[serde(rename = "vcs.write")]
    VcsWrite,
    #[serde(rename = "assistant.read")]
    AssistantRead,
    #[serde(rename = "tokens.read")]
    TokensRead,
    #[serde(rename = "tokens.write")]
    TokensWrite,
    #[serde(rename = "webhooks.read")]
    WebhooksRead,
    #[serde(rename = "webhooks.write")]
    WebhooksWrite,
    #[serde(rename = "project.read")]
    ProjectRead,
    #[serde(rename = "project.write")]
    ProjectWrite,
    #[serde(rename = "policies.write")]
    PoliciesWrite,
    #[serde(rename = "policies.read")]
    PoliciesRead,
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
    #[serde(rename = "events.read")]
    EventsRead,
}

impl Scopes {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Scopes::SessionsWrite => "sessions.write",
            Scopes::UsersRead => "users.read",
            Scopes::UsersWrite => "users.write",
            Scopes::TeamsRead => "teams.read",
            Scopes::TeamsWrite => "teams.write",
            Scopes::DatabasesRead => "databases.read",
            Scopes::DatabasesWrite => "databases.write",
            Scopes::CollectionsRead => "collections.read",
            Scopes::CollectionsWrite => "collections.write",
            Scopes::TablesRead => "tables.read",
            Scopes::TablesWrite => "tables.write",
            Scopes::AttributesRead => "attributes.read",
            Scopes::AttributesWrite => "attributes.write",
            Scopes::ColumnsRead => "columns.read",
            Scopes::ColumnsWrite => "columns.write",
            Scopes::IndexesRead => "indexes.read",
            Scopes::IndexesWrite => "indexes.write",
            Scopes::DocumentsRead => "documents.read",
            Scopes::DocumentsWrite => "documents.write",
            Scopes::RowsRead => "rows.read",
            Scopes::RowsWrite => "rows.write",
            Scopes::FilesRead => "files.read",
            Scopes::FilesWrite => "files.write",
            Scopes::BucketsRead => "buckets.read",
            Scopes::BucketsWrite => "buckets.write",
            Scopes::FunctionsRead => "functions.read",
            Scopes::FunctionsWrite => "functions.write",
            Scopes::SitesRead => "sites.read",
            Scopes::SitesWrite => "sites.write",
            Scopes::LogRead => "log.read",
            Scopes::LogWrite => "log.write",
            Scopes::ExecutionRead => "execution.read",
            Scopes::ExecutionWrite => "execution.write",
            Scopes::LocaleRead => "locale.read",
            Scopes::AvatarsRead => "avatars.read",
            Scopes::HealthRead => "health.read",
            Scopes::ProvidersRead => "providers.read",
            Scopes::ProvidersWrite => "providers.write",
            Scopes::MessagesRead => "messages.read",
            Scopes::MessagesWrite => "messages.write",
            Scopes::TopicsRead => "topics.read",
            Scopes::TopicsWrite => "topics.write",
            Scopes::SubscribersRead => "subscribers.read",
            Scopes::SubscribersWrite => "subscribers.write",
            Scopes::TargetsRead => "targets.read",
            Scopes::TargetsWrite => "targets.write",
            Scopes::RulesRead => "rules.read",
            Scopes::RulesWrite => "rules.write",
            Scopes::SchedulesRead => "schedules.read",
            Scopes::SchedulesWrite => "schedules.write",
            Scopes::MigrationsRead => "migrations.read",
            Scopes::MigrationsWrite => "migrations.write",
            Scopes::VcsRead => "vcs.read",
            Scopes::VcsWrite => "vcs.write",
            Scopes::AssistantRead => "assistant.read",
            Scopes::TokensRead => "tokens.read",
            Scopes::TokensWrite => "tokens.write",
            Scopes::WebhooksRead => "webhooks.read",
            Scopes::WebhooksWrite => "webhooks.write",
            Scopes::ProjectRead => "project.read",
            Scopes::ProjectWrite => "project.write",
            Scopes::PoliciesWrite => "policies.write",
            Scopes::PoliciesRead => "policies.read",
            Scopes::ArchivesRead => "archives.read",
            Scopes::ArchivesWrite => "archives.write",
            Scopes::RestorationsRead => "restorations.read",
            Scopes::RestorationsWrite => "restorations.write",
            Scopes::DomainsRead => "domains.read",
            Scopes::DomainsWrite => "domains.write",
            Scopes::EventsRead => "events.read",
        }
    }
}

impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
