use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Scopes {
    #[serde(rename = "project.read")]
    #[default]
    ProjectRead,
    #[serde(rename = "project.write")]
    ProjectWrite,
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
    #[serde(rename = "templates.read")]
    TemplatesRead,
    #[serde(rename = "templates.write")]
    TemplatesWrite,
    #[serde(rename = "oauth2.read")]
    Oauth2Read,
    #[serde(rename = "oauth2.write")]
    Oauth2Write,
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
    #[serde(rename = "events.read")]
    EventsRead,
}

impl Scopes {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Scopes::ProjectRead => "project.read",
            Scopes::ProjectWrite => "project.write",
            Scopes::KeysRead => "keys.read",
            Scopes::KeysWrite => "keys.write",
            Scopes::PlatformsRead => "platforms.read",
            Scopes::PlatformsWrite => "platforms.write",
            Scopes::MocksRead => "mocks.read",
            Scopes::MocksWrite => "mocks.write",
            Scopes::PoliciesRead => "policies.read",
            Scopes::PoliciesWrite => "policies.write",
            Scopes::ProjectPoliciesRead => "project.policies.read",
            Scopes::ProjectPoliciesWrite => "project.policies.write",
            Scopes::TemplatesRead => "templates.read",
            Scopes::TemplatesWrite => "templates.write",
            Scopes::Oauth2Read => "oauth2.read",
            Scopes::Oauth2Write => "oauth2.write",
            Scopes::UsersRead => "users.read",
            Scopes::UsersWrite => "users.write",
            Scopes::SessionsRead => "sessions.read",
            Scopes::SessionsWrite => "sessions.write",
            Scopes::TeamsRead => "teams.read",
            Scopes::TeamsWrite => "teams.write",
            Scopes::DatabasesRead => "databases.read",
            Scopes::DatabasesWrite => "databases.write",
            Scopes::TablesRead => "tables.read",
            Scopes::TablesWrite => "tables.write",
            Scopes::ColumnsRead => "columns.read",
            Scopes::ColumnsWrite => "columns.write",
            Scopes::IndexesRead => "indexes.read",
            Scopes::IndexesWrite => "indexes.write",
            Scopes::RowsRead => "rows.read",
            Scopes::RowsWrite => "rows.write",
            Scopes::CollectionsRead => "collections.read",
            Scopes::CollectionsWrite => "collections.write",
            Scopes::AttributesRead => "attributes.read",
            Scopes::AttributesWrite => "attributes.write",
            Scopes::DocumentsRead => "documents.read",
            Scopes::DocumentsWrite => "documents.write",
            Scopes::BucketsRead => "buckets.read",
            Scopes::BucketsWrite => "buckets.write",
            Scopes::FilesRead => "files.read",
            Scopes::FilesWrite => "files.write",
            Scopes::TokensRead => "tokens.read",
            Scopes::TokensWrite => "tokens.write",
            Scopes::FunctionsRead => "functions.read",
            Scopes::FunctionsWrite => "functions.write",
            Scopes::ExecutionsRead => "executions.read",
            Scopes::ExecutionsWrite => "executions.write",
            Scopes::ExecutionRead => "execution.read",
            Scopes::ExecutionWrite => "execution.write",
            Scopes::SitesRead => "sites.read",
            Scopes::SitesWrite => "sites.write",
            Scopes::LogRead => "log.read",
            Scopes::LogWrite => "log.write",
            Scopes::ProvidersRead => "providers.read",
            Scopes::ProvidersWrite => "providers.write",
            Scopes::TopicsRead => "topics.read",
            Scopes::TopicsWrite => "topics.write",
            Scopes::SubscribersRead => "subscribers.read",
            Scopes::SubscribersWrite => "subscribers.write",
            Scopes::TargetsRead => "targets.read",
            Scopes::TargetsWrite => "targets.write",
            Scopes::MessagesRead => "messages.read",
            Scopes::MessagesWrite => "messages.write",
            Scopes::RulesRead => "rules.read",
            Scopes::RulesWrite => "rules.write",
            Scopes::WebhooksRead => "webhooks.read",
            Scopes::WebhooksWrite => "webhooks.write",
            Scopes::LocaleRead => "locale.read",
            Scopes::AvatarsRead => "avatars.read",
            Scopes::HealthRead => "health.read",
            Scopes::AssistantRead => "assistant.read",
            Scopes::MigrationsRead => "migrations.read",
            Scopes::MigrationsWrite => "migrations.write",
            Scopes::SchedulesRead => "schedules.read",
            Scopes::SchedulesWrite => "schedules.write",
            Scopes::VcsRead => "vcs.read",
            Scopes::VcsWrite => "vcs.write",
            Scopes::PresencesRead => "presences.read",
            Scopes::PresencesWrite => "presences.write",
            Scopes::BackupsPoliciesRead => "backups.policies.read",
            Scopes::BackupsPoliciesWrite => "backups.policies.write",
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
