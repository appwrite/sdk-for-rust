//! DedicatedDatabaseBranch model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBranch {
    /// Branch identifier.
    #[serde(rename = "branchId")]
    pub branch_id: String,
    /// Branch name.
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// Kubernetes namespace where the branch is deployed.
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Unix timestamp when the branch expires.
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
    /// Branch hostname for direct connections.
    #[serde(rename = "host")]
    pub host: String,
    /// Branch port. Null until the backing reports one.
    #[serde(rename = "port")]
    pub port: i64,
    /// Advertised catalog the client connects to. MySQL/MariaDB use default;
    /// Postgres uses the routing label.
    #[serde(rename = "database")]
    pub database: String,
    /// Database username. Shared with the parent database.
    #[serde(rename = "username")]
    pub username: String,
    /// Database password. Shared with the parent database.
    #[serde(rename = "password")]
    pub password: String,
    /// Whether SSL is required.
    #[serde(rename = "ssl")]
    pub ssl: bool,
    /// Database engine. Possible values: postgresql, mysql, mongodb.
    #[serde(rename = "engine")]
    pub engine: String,
    /// Full connection string for the branch.
    #[serde(rename = "connectionString")]
    pub connection_string: String,
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

    /// Get port
    pub fn port(&self) -> &i64 {
        &self.port
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
        let _ = _model.port();
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
