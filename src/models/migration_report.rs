//! MigrationReport model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Migration Report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct MigrationReport {
    /// Number of users to be migrated.
    #[serde(rename = "user")]
    #[serde(default = "default_user")]
    pub user: i64,
    /// Number of teams to be migrated.
    #[serde(rename = "team")]
    #[serde(default = "default_team")]
    pub team: i64,
    /// Number of databases to be migrated.
    #[serde(rename = "database")]
    #[serde(default = "default_database")]
    pub database: i64,
    /// Number of rows to be migrated.
    #[serde(rename = "row")]
    #[serde(default = "default_row")]
    pub row: i64,
    /// Number of files to be migrated.
    #[serde(rename = "file")]
    #[serde(default = "default_file")]
    pub file: i64,
    /// Number of buckets to be migrated.
    #[serde(rename = "bucket")]
    #[serde(default = "default_bucket")]
    pub bucket: i64,
    /// Number of functions to be migrated.
    #[serde(rename = "function")]
    #[serde(default = "default_function")]
    pub function: i64,
    /// Number of platforms to be migrated.
    #[serde(rename = "platform")]
    #[serde(default = "default_platform")]
    pub platform: i64,
    /// Number of API keys to be migrated.
    #[serde(rename = "api-key")]
    #[serde(default = "default_api_key")]
    pub api_key: i64,
    /// Number of project variables to be migrated.
    #[serde(rename = "project-variable")]
    #[serde(default = "default_project_variable")]
    pub project_variable: i64,
    /// Number of webhooks to be migrated.
    #[serde(rename = "webhook")]
    #[serde(default = "default_webhook")]
    pub webhook: i64,
    /// Number of auth-method configs to be migrated (always 0 or 1 — the
    /// project-level flag bundle).
    #[serde(rename = "auth-methods")]
    #[serde(default = "default_auth_methods")]
    pub auth_methods: i64,
    /// Number of protocol configs to be migrated (always 0 or 1 — the
    /// project-level REST/GraphQL/WebSocket flags).
    #[serde(rename = "project-protocols")]
    #[serde(default = "default_project_protocols")]
    pub project_protocols: i64,
    /// Number of label sets to be migrated (always 0 or 1 — the project-level
    /// RBAC label array).
    #[serde(rename = "project-labels")]
    #[serde(default = "default_project_labels")]
    pub project_labels: i64,
    /// Number of service configs to be migrated (always 0 or 1 — the
    /// project-level enable/disable flags for all 17 services).
    #[serde(rename = "project-services")]
    #[serde(default = "default_project_services")]
    pub project_services: i64,
    /// Number of policy bundles to be migrated (always 0 or 1 — the
    /// project-level security policies covering password rules, session behavior,
    /// user limits, and membership privacy).
    #[serde(rename = "policies")]
    #[serde(default = "default_policies")]
    pub policies: i64,
    /// Number of SMTP configurations to be migrated (always 0 or 1 — the
    /// project-level custom SMTP settings; password is not exposed by the source
    /// API).
    #[serde(rename = "smtp")]
    #[serde(default = "default_smtp")]
    pub smtp: i64,
    /// Number of custom-domain proxy rules to be migrated. Auto-generated
    /// `.appwrite.network` rules are skipped — they are recreated by parent
    /// Function/Site migration.
    #[serde(rename = "rule")]
    #[serde(default = "default_rule")]
    pub rule: i64,
    /// Number of custom email templates to be migrated (one per templateId ×
    /// locale pair).
    #[serde(rename = "project-email-template")]
    #[serde(default = "default_project_email_template")]
    pub project_email_template: i64,
    /// Number of project environments to be migrated. Each is recreated under a
    /// new ID, empty: its own resources belong to a separate project.
    #[serde(rename = "project-environment")]
    #[serde(default = "default_project_environment")]
    pub project_environment: i64,
    /// Number of sites to be migrated.
    #[serde(rename = "site")]
    #[serde(default = "default_site")]
    pub site: i64,
    /// Number of providers to be migrated.
    #[serde(rename = "provider")]
    #[serde(default = "default_provider")]
    pub provider: i64,
    /// Number of topics to be migrated.
    #[serde(rename = "topic")]
    #[serde(default = "default_topic")]
    pub topic: i64,
    /// Number of subscribers to be migrated.
    #[serde(rename = "subscriber")]
    #[serde(default = "default_subscriber")]
    pub subscriber: i64,
    /// Number of messages to be migrated.
    #[serde(rename = "message")]
    #[serde(default = "default_message")]
    pub message: i64,
    /// Size of files to be migrated in mb.
    #[serde(rename = "size")]
    #[serde(default = "default_size")]
    pub size: i64,
    /// Version of the Appwrite instance to be migrated.
    #[serde(rename = "version")]
    #[serde(default = "default_version")]
    pub version: String,
    /// Number of OAuth2 provider configurations to be migrated. Secrets
    /// (clientSecret, p8File) are never migrated — destination admin must
    /// re-enter them per provider.
    #[serde(rename = "oauth2-provider")]
    #[serde(default = "default_oauth2_provider")]
    pub oauth2_provider: i64,
    /// Number of backup policies to be migrated.
    #[serde(rename = "backup-policy")]
    #[serde(default = "default_backup_policy")]
    pub backup_policy: i64,
}

fn default_user() -> i64 {
    0
}

fn default_team() -> i64 {
    0
}

fn default_database() -> i64 {
    0
}

fn default_row() -> i64 {
    0
}

fn default_file() -> i64 {
    0
}

fn default_bucket() -> i64 {
    0
}

fn default_function() -> i64 {
    0
}

fn default_platform() -> i64 {
    0
}

fn default_api_key() -> i64 {
    0
}

fn default_project_variable() -> i64 {
    0
}

fn default_webhook() -> i64 {
    0
}

fn default_auth_methods() -> i64 {
    0
}

fn default_project_protocols() -> i64 {
    0
}

fn default_project_labels() -> i64 {
    0
}

fn default_project_services() -> i64 {
    0
}

fn default_policies() -> i64 {
    0
}

fn default_smtp() -> i64 {
    0
}

fn default_rule() -> i64 {
    0
}

fn default_project_email_template() -> i64 {
    0
}

fn default_project_environment() -> i64 {
    0
}

fn default_site() -> i64 {
    0
}

fn default_provider() -> i64 {
    0
}

fn default_topic() -> i64 {
    0
}

fn default_subscriber() -> i64 {
    0
}

fn default_message() -> i64 {
    0
}

fn default_size() -> i64 {
    0
}

fn default_version() -> String {
    "".to_string()
}

fn default_oauth2_provider() -> i64 {
    0
}

fn default_backup_policy() -> i64 {
    0
}

impl MigrationReport {
    /// Get user
    pub fn user(&self) -> &i64 {
        &self.user
    }

    /// Get team
    pub fn team(&self) -> &i64 {
        &self.team
    }

    /// Get database
    pub fn database(&self) -> &i64 {
        &self.database
    }

    /// Get row
    pub fn row(&self) -> &i64 {
        &self.row
    }

    /// Get file
    pub fn file(&self) -> &i64 {
        &self.file
    }

    /// Get bucket
    pub fn bucket(&self) -> &i64 {
        &self.bucket
    }

    /// Get function
    pub fn function(&self) -> &i64 {
        &self.function
    }

    /// Get platform
    pub fn platform(&self) -> &i64 {
        &self.platform
    }

    /// Get api_key
    pub fn api_key(&self) -> &i64 {
        &self.api_key
    }

    /// Get project_variable
    pub fn project_variable(&self) -> &i64 {
        &self.project_variable
    }

    /// Get webhook
    pub fn webhook(&self) -> &i64 {
        &self.webhook
    }

    /// Get auth_methods
    pub fn auth_methods(&self) -> &i64 {
        &self.auth_methods
    }

    /// Get project_protocols
    pub fn project_protocols(&self) -> &i64 {
        &self.project_protocols
    }

    /// Get project_labels
    pub fn project_labels(&self) -> &i64 {
        &self.project_labels
    }

    /// Get project_services
    pub fn project_services(&self) -> &i64 {
        &self.project_services
    }

    /// Get policies
    pub fn policies(&self) -> &i64 {
        &self.policies
    }

    /// Get smtp
    pub fn smtp(&self) -> &i64 {
        &self.smtp
    }

    /// Get rule
    pub fn rule(&self) -> &i64 {
        &self.rule
    }

    /// Get project_email_template
    pub fn project_email_template(&self) -> &i64 {
        &self.project_email_template
    }

    /// Get project_environment
    pub fn project_environment(&self) -> &i64 {
        &self.project_environment
    }

    /// Get site
    pub fn site(&self) -> &i64 {
        &self.site
    }

    /// Get provider
    pub fn provider(&self) -> &i64 {
        &self.provider
    }

    /// Get topic
    pub fn topic(&self) -> &i64 {
        &self.topic
    }

    /// Get subscriber
    pub fn subscriber(&self) -> &i64 {
        &self.subscriber
    }

    /// Get message
    pub fn message(&self) -> &i64 {
        &self.message
    }

    /// Get size
    pub fn size(&self) -> &i64 {
        &self.size
    }

    /// Get version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get oauth2_provider
    pub fn oauth2_provider(&self) -> &i64 {
        &self.oauth2_provider
    }

    /// Get backup_policy
    pub fn backup_policy(&self) -> &i64 {
        &self.backup_policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_report_creation() {
        let _model = <MigrationReport as Default>::default();
        let _ = _model.user();
        let _ = _model.team();
        let _ = _model.database();
        let _ = _model.row();
        let _ = _model.file();
        let _ = _model.bucket();
        let _ = _model.function();
        let _ = _model.platform();
        let _ = _model.api_key();
        let _ = _model.project_variable();
        let _ = _model.webhook();
        let _ = _model.auth_methods();
        let _ = _model.project_protocols();
        let _ = _model.project_labels();
        let _ = _model.project_services();
        let _ = _model.policies();
        let _ = _model.smtp();
        let _ = _model.rule();
        let _ = _model.project_email_template();
        let _ = _model.project_environment();
        let _ = _model.site();
        let _ = _model.provider();
        let _ = _model.topic();
        let _ = _model.subscriber();
        let _ = _model.message();
        let _ = _model.size();
        let _ = _model.version();
        let _ = _model.oauth2_provider();
        let _ = _model.backup_policy();
    }

    #[test]
    fn test_migration_report_serialization() {
        let model = <MigrationReport as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<MigrationReport, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
