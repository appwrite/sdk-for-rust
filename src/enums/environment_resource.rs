use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EnvironmentResource {
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
    #[serde(rename = "documentsdb")]
    Documentsdb,
    #[serde(rename = "vectorsdb")]
    Vectorsdb,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "collection")]
    Collection,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "bucket")]
    Bucket,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "environment-variable")]
    EnvironmentVariable,
    #[serde(rename = "deployment")]
    Deployment,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "site-variable")]
    SiteVariable,
    #[serde(rename = "site-deployment")]
    SiteDeployment,
    #[serde(rename = "provider")]
    Provider,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "subscriber")]
    Subscriber,
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "smtp")]
    Smtp,
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
}

impl EnvironmentResource {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            EnvironmentResource::User => "user",
            EnvironmentResource::Team => "team",
            EnvironmentResource::Membership => "membership",
            EnvironmentResource::AuthMethods => "auth-methods",
            EnvironmentResource::Policies => "policies",
            EnvironmentResource::Oauth2Provider => "oauth2-provider",
            EnvironmentResource::Database => "database",
            EnvironmentResource::Documentsdb => "documentsdb",
            EnvironmentResource::Vectorsdb => "vectorsdb",
            EnvironmentResource::Table => "table",
            EnvironmentResource::Collection => "collection",
            EnvironmentResource::Column => "column",
            EnvironmentResource::Attribute => "attribute",
            EnvironmentResource::Index => "index",
            EnvironmentResource::Row => "row",
            EnvironmentResource::Document => "document",
            EnvironmentResource::Bucket => "bucket",
            EnvironmentResource::File => "file",
            EnvironmentResource::Function => "function",
            EnvironmentResource::EnvironmentVariable => "environment-variable",
            EnvironmentResource::Deployment => "deployment",
            EnvironmentResource::Site => "site",
            EnvironmentResource::SiteVariable => "site-variable",
            EnvironmentResource::SiteDeployment => "site-deployment",
            EnvironmentResource::Provider => "provider",
            EnvironmentResource::Topic => "topic",
            EnvironmentResource::Subscriber => "subscriber",
            EnvironmentResource::Platform => "platform",
            EnvironmentResource::Smtp => "smtp",
            EnvironmentResource::ProjectVariable => "project-variable",
            EnvironmentResource::ProjectProtocols => "project-protocols",
            EnvironmentResource::ProjectLabels => "project-labels",
            EnvironmentResource::ProjectServices => "project-services",
            EnvironmentResource::ProjectEmailTemplate => "project-email-template",
        }
    }
}

impl std::fmt::Display for EnvironmentResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
