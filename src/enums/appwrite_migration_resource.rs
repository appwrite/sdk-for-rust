use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AppwriteMigrationResource {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "membership")]
    Membership,
    #[serde(rename = "auth-methods")]
    AuthMethods,
    #[serde(rename = "policies")]
    Policies,
    #[serde(rename = "oauth2-provider")]
    Oauth2Provider,
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
    #[serde(rename = "documentsdb")]
    Documentsdb,
    #[serde(rename = "vectorsdb")]
    Vectorsdb,
    #[serde(rename = "bucket")]
    Bucket,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "deployment")]
    Deployment,
    #[serde(rename = "environment-variable")]
    EnvironmentVariable,
    #[serde(rename = "provider")]
    Provider,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "subscriber")]
    Subscriber,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "site-deployment")]
    SiteDeployment,
    #[serde(rename = "site-variable")]
    SiteVariable,
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "api-key")]
    ApiKey,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "smtp")]
    Smtp,
    #[serde(rename = "backup-policy")]
    BackupPolicy,
    #[serde(rename = "project-variable")]
    ProjectVariable,
    #[serde(rename = "project-protocols")]
    ProjectProtocols,
    #[serde(rename = "project-labels")]
    ProjectLabels,
    #[serde(rename = "project-services")]
    ProjectServices,
    #[serde(rename = "project-email-template")]
    ProjectEmailTemplate,
    #[serde(rename = "project-environment")]
    ProjectEnvironment,
    #[serde(rename = "rule")]
    Rule,
}

impl AppwriteMigrationResource {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AppwriteMigrationResource::User => "user",
            AppwriteMigrationResource::Team => "team",
            AppwriteMigrationResource::Membership => "membership",
            AppwriteMigrationResource::AuthMethods => "auth-methods",
            AppwriteMigrationResource::Policies => "policies",
            AppwriteMigrationResource::Oauth2Provider => "oauth2-provider",
            AppwriteMigrationResource::Database => "database",
            AppwriteMigrationResource::Table => "table",
            AppwriteMigrationResource::Column => "column",
            AppwriteMigrationResource::Index => "index",
            AppwriteMigrationResource::Row => "row",
            AppwriteMigrationResource::Document => "document",
            AppwriteMigrationResource::Attribute => "attribute",
            AppwriteMigrationResource::Collection => "collection",
            AppwriteMigrationResource::Documentsdb => "documentsdb",
            AppwriteMigrationResource::Vectorsdb => "vectorsdb",
            AppwriteMigrationResource::Bucket => "bucket",
            AppwriteMigrationResource::File => "file",
            AppwriteMigrationResource::Function => "function",
            AppwriteMigrationResource::Deployment => "deployment",
            AppwriteMigrationResource::EnvironmentVariable => "environment-variable",
            AppwriteMigrationResource::Provider => "provider",
            AppwriteMigrationResource::Topic => "topic",
            AppwriteMigrationResource::Subscriber => "subscriber",
            AppwriteMigrationResource::Message => "message",
            AppwriteMigrationResource::Site => "site",
            AppwriteMigrationResource::SiteDeployment => "site-deployment",
            AppwriteMigrationResource::SiteVariable => "site-variable",
            AppwriteMigrationResource::Platform => "platform",
            AppwriteMigrationResource::ApiKey => "api-key",
            AppwriteMigrationResource::Webhook => "webhook",
            AppwriteMigrationResource::Smtp => "smtp",
            AppwriteMigrationResource::BackupPolicy => "backup-policy",
            AppwriteMigrationResource::ProjectVariable => "project-variable",
            AppwriteMigrationResource::ProjectProtocols => "project-protocols",
            AppwriteMigrationResource::ProjectLabels => "project-labels",
            AppwriteMigrationResource::ProjectServices => "project-services",
            AppwriteMigrationResource::ProjectEmailTemplate => "project-email-template",
            AppwriteMigrationResource::ProjectEnvironment => "project-environment",
            AppwriteMigrationResource::Rule => "rule",
        }
    }
}

impl std::fmt::Display for AppwriteMigrationResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
