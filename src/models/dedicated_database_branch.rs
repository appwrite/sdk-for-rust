//! DedicatedDatabaseBranch model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBranch {
    /// Branch identifier.
    #[serde(rename = "branchId")]
    #[serde(default = "default_branch_id")]
    pub branch_id: String,
    /// Branch name.
    #[serde(rename = "branchName")]
    #[serde(default = "default_branch_name")]
    pub branch_name: String,
    /// Kubernetes namespace where the branch is deployed.
    #[serde(rename = "namespace")]
    #[serde(default = "default_namespace")]
    pub namespace: String,
    /// Unix timestamp when the branch expires.
    #[serde(rename = "expiresAt")]
    #[serde(default = "default_expires_at")]
    pub expires_at: i64,
    /// Branch hostname for direct connections.
    #[serde(rename = "host")]
    #[serde(default = "default_host")]
    pub host: String,
    /// Branch port. Null until the backing reports one.
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Advertised catalog the client connects to. MySQL/MariaDB use default;
    /// Postgres uses the routing label.
    #[serde(rename = "database")]
    #[serde(default = "default_database")]
    pub database: String,
    /// Database username. Shared with the parent database.
    #[serde(rename = "username")]
    #[serde(default = "default_username")]
    pub username: String,
    /// Database password. Shared with the parent database.
    #[serde(rename = "password")]
    #[serde(default = "default_password")]
    pub password: String,
    /// Whether SSL is required.
    #[serde(rename = "ssl")]
    #[serde(default = "default_ssl")]
    pub ssl: bool,
    /// Database engine. Possible values: postgresql, mysql, mongodb.
    #[serde(rename = "engine")]
    #[serde(default = "default_engine")]
    pub engine: String,
    /// Full connection string for the branch.
    #[serde(rename = "connectionString")]
    #[serde(default = "default_connection_string")]
    pub connection_string: String,
}

fn default_branch_id() -> String {
    "".to_string()
}

fn default_branch_name() -> String {
    "".to_string()
}

fn default_namespace() -> String {
    "".to_string()
}

fn default_expires_at() -> i64 {
    0
}

fn default_host() -> String {
    "".to_string()
}

fn default_database() -> String {
    "".to_string()
}

fn default_username() -> String {
    "".to_string()
}

fn default_password() -> String {
    "".to_string()
}

fn default_ssl() -> bool {
    false
}

fn default_engine() -> String {
    "".to_string()
}

fn default_connection_string() -> String {
    "".to_string()
}

impl DedicatedDatabaseBranch {
    /// Get branch_id
    pub fn branch_id(&self) -> &String {
        &self.branch_id
    }

    /// Get branch_name
    pub fn branch_name(&self) -> &String {
        &self.branch_name
    }

    /// Get namespace
    pub fn namespace(&self) -> &String {
        &self.namespace
    }

    /// Get expires_at
    pub fn expires_at(&self) -> &i64 {
        &self.expires_at
    }

    /// Get host
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Set port
    pub fn set_port(mut self, port: i64) -> Self {
        self.port = Some(port);
        self
    }

    /// Get port
    pub fn port(&self) -> Option<&i64> {
        self.port.as_ref()
    }

    /// Get database
    pub fn database(&self) -> &String {
        &self.database
    }

    /// Get username
    pub fn username(&self) -> &String {
        &self.username
    }

    /// Get password
    pub fn password(&self) -> &String {
        &self.password
    }

    /// Get ssl
    pub fn ssl(&self) -> &bool {
        &self.ssl
    }

    /// Get engine
    pub fn engine(&self) -> &String {
        &self.engine
    }

    /// Get connection_string
    pub fn connection_string(&self) -> &String {
        &self.connection_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_branch_creation() {
        let _model = <DedicatedDatabaseBranch as Default>::default();
        let _ = _model.branch_id();
        let _ = _model.branch_name();
        let _ = _model.namespace();
        let _ = _model.expires_at();
        let _ = _model.host();
        let _ = _model.database();
        let _ = _model.username();
        let _ = _model.password();
        let _ = _model.ssl();
        let _ = _model.engine();
        let _ = _model.connection_string();
    }

    #[test]
    fn test_dedicated_database_branch_serialization() {
        let model = <DedicatedDatabaseBranch as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseBranch, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
