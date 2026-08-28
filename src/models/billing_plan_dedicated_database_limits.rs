//! BillingPlanDedicatedDatabaseLimits model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// dedicatedDatabaseLimits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlanDedicatedDatabaseLimits {
    /// Minimum CPU allocation in millicores.
    #[serde(rename = "minCpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cpu: Option<i64>,
    /// Maximum CPU allocation in millicores.
    #[serde(rename = "maxCpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu: Option<i64>,
    /// Minimum memory allocation in megabytes.
    #[serde(rename = "minMemoryMb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_memory_mb: Option<i64>,
    /// Maximum memory allocation in megabytes.
    #[serde(rename = "maxMemoryMb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<i64>,
    /// Minimum storage allocation in gigabytes.
    #[serde(rename = "minStorageGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_gb: Option<i64>,
    /// Maximum storage allocation in gigabytes.
    #[serde(rename = "maxStorageGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_gb: Option<i64>,
    /// Maximum number of high-availability replicas per dedicated database.
    #[serde(rename = "maxReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i64>,
    /// Maximum number of client connections.
    #[serde(rename = "maxConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// Maximum number of entries allowed in the IP allowlist.
    #[serde(rename = "maxIpAllowlistSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ip_allowlist_size: Option<i64>,
    /// Maximum number of database extensions that can be enabled.
    #[serde(rename = "maxExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_extensions: Option<i64>,
    /// Maximum number of days a backup can be retained.
    #[serde(rename = "maxBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i64>,
    /// Maximum number of days of point-in-time recovery data that can be retained.
    #[serde(rename = "maxPitrRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pitr_retention_days: Option<i64>,
    /// Maximum number of rows a single SQL API query can return.
    #[serde(rename = "maxSqlApiMaxRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sql_api_max_rows: Option<i64>,
    /// Maximum response size in bytes for a single SQL API query.
    #[serde(rename = "maxSqlApiMaxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sql_api_max_bytes: Option<i64>,
    /// Maximum execution time in seconds for a single SQL API query.
    #[serde(rename = "maxSqlApiTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sql_api_timeout_seconds: Option<i64>,
    /// Maximum number of SQL statement types that can be permitted through the SQL
    /// API.
    #[serde(rename = "maxSqlApiAllowedStatements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sql_api_allowed_statements: Option<i64>,
    /// SQL statement types permitted through the SQL API.
    #[serde(rename = "allowedSqlStatements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sql_statements: Option<Vec<String>>,
    /// Storage classes available for dedicated databases.
    #[serde(rename = "allowedStorageClasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_storage_classes: Option<Vec<String>>,
    /// Replica synchronization modes available for dedicated databases.
    #[serde(rename = "allowedSyncModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sync_modes: Option<Vec<String>>,
}

impl BillingPlanDedicatedDatabaseLimits {
    /// Set min_cpu
    pub fn set_min_cpu(mut self, min_cpu: i64) -> Self {
        self.min_cpu = Some(min_cpu);
        self
    }

    /// Get min_cpu
    pub fn min_cpu(&self) -> Option<&i64> {
        self.min_cpu.as_ref()
    }

    /// Set max_cpu
    pub fn set_max_cpu(mut self, max_cpu: i64) -> Self {
        self.max_cpu = Some(max_cpu);
        self
    }

    /// Get max_cpu
    pub fn max_cpu(&self) -> Option<&i64> {
        self.max_cpu.as_ref()
    }

    /// Set min_memory_mb
    pub fn set_min_memory_mb(mut self, min_memory_mb: i64) -> Self {
        self.min_memory_mb = Some(min_memory_mb);
        self
    }

    /// Get min_memory_mb
    pub fn min_memory_mb(&self) -> Option<&i64> {
        self.min_memory_mb.as_ref()
    }

    /// Set max_memory_mb
    pub fn set_max_memory_mb(mut self, max_memory_mb: i64) -> Self {
        self.max_memory_mb = Some(max_memory_mb);
        self
    }

    /// Get max_memory_mb
    pub fn max_memory_mb(&self) -> Option<&i64> {
        self.max_memory_mb.as_ref()
    }

    /// Set min_storage_gb
    pub fn set_min_storage_gb(mut self, min_storage_gb: i64) -> Self {
        self.min_storage_gb = Some(min_storage_gb);
        self
    }

    /// Get min_storage_gb
    pub fn min_storage_gb(&self) -> Option<&i64> {
        self.min_storage_gb.as_ref()
    }

    /// Set max_storage_gb
    pub fn set_max_storage_gb(mut self, max_storage_gb: i64) -> Self {
        self.max_storage_gb = Some(max_storage_gb);
        self
    }

    /// Get max_storage_gb
    pub fn max_storage_gb(&self) -> Option<&i64> {
        self.max_storage_gb.as_ref()
    }

    /// Set max_replicas
    pub fn set_max_replicas(mut self, max_replicas: i64) -> Self {
        self.max_replicas = Some(max_replicas);
        self
    }

    /// Get max_replicas
    pub fn max_replicas(&self) -> Option<&i64> {
        self.max_replicas.as_ref()
    }

    /// Set max_connections
    pub fn set_max_connections(mut self, max_connections: i64) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    /// Get max_connections
    pub fn max_connections(&self) -> Option<&i64> {
        self.max_connections.as_ref()
    }

    /// Set max_ip_allowlist_size
    pub fn set_max_ip_allowlist_size(mut self, max_ip_allowlist_size: i64) -> Self {
        self.max_ip_allowlist_size = Some(max_ip_allowlist_size);
        self
    }

    /// Get max_ip_allowlist_size
    pub fn max_ip_allowlist_size(&self) -> Option<&i64> {
        self.max_ip_allowlist_size.as_ref()
    }

    /// Set max_extensions
    pub fn set_max_extensions(mut self, max_extensions: i64) -> Self {
        self.max_extensions = Some(max_extensions);
        self
    }

    /// Get max_extensions
    pub fn max_extensions(&self) -> Option<&i64> {
        self.max_extensions.as_ref()
    }

    /// Set max_backup_retention_days
    pub fn set_max_backup_retention_days(mut self, max_backup_retention_days: i64) -> Self {
        self.max_backup_retention_days = Some(max_backup_retention_days);
        self
    }

    /// Get max_backup_retention_days
    pub fn max_backup_retention_days(&self) -> Option<&i64> {
        self.max_backup_retention_days.as_ref()
    }

    /// Set max_pitr_retention_days
    pub fn set_max_pitr_retention_days(mut self, max_pitr_retention_days: i64) -> Self {
        self.max_pitr_retention_days = Some(max_pitr_retention_days);
        self
    }

    /// Get max_pitr_retention_days
    pub fn max_pitr_retention_days(&self) -> Option<&i64> {
        self.max_pitr_retention_days.as_ref()
    }

    /// Set max_sql_api_max_rows
    pub fn set_max_sql_api_max_rows(mut self, max_sql_api_max_rows: i64) -> Self {
        self.max_sql_api_max_rows = Some(max_sql_api_max_rows);
        self
    }

    /// Get max_sql_api_max_rows
    pub fn max_sql_api_max_rows(&self) -> Option<&i64> {
        self.max_sql_api_max_rows.as_ref()
    }

    /// Set max_sql_api_max_bytes
    pub fn set_max_sql_api_max_bytes(mut self, max_sql_api_max_bytes: i64) -> Self {
        self.max_sql_api_max_bytes = Some(max_sql_api_max_bytes);
        self
    }

    /// Get max_sql_api_max_bytes
    pub fn max_sql_api_max_bytes(&self) -> Option<&i64> {
        self.max_sql_api_max_bytes.as_ref()
    }

    /// Set max_sql_api_timeout_seconds
    pub fn set_max_sql_api_timeout_seconds(mut self, max_sql_api_timeout_seconds: i64) -> Self {
        self.max_sql_api_timeout_seconds = Some(max_sql_api_timeout_seconds);
        self
    }

    /// Get max_sql_api_timeout_seconds
    pub fn max_sql_api_timeout_seconds(&self) -> Option<&i64> {
        self.max_sql_api_timeout_seconds.as_ref()
    }

    /// Set max_sql_api_allowed_statements
    pub fn set_max_sql_api_allowed_statements(
        mut self,
        max_sql_api_allowed_statements: i64,
    ) -> Self {
        self.max_sql_api_allowed_statements = Some(max_sql_api_allowed_statements);
        self
    }

    /// Get max_sql_api_allowed_statements
    pub fn max_sql_api_allowed_statements(&self) -> Option<&i64> {
        self.max_sql_api_allowed_statements.as_ref()
    }

    /// Set allowed_sql_statements
    pub fn set_allowed_sql_statements(mut self, allowed_sql_statements: Vec<String>) -> Self {
        self.allowed_sql_statements = Some(allowed_sql_statements);
        self
    }

    /// Get allowed_sql_statements
    pub fn allowed_sql_statements(&self) -> Option<&Vec<String>> {
        self.allowed_sql_statements.as_ref()
    }

    /// Set allowed_storage_classes
    pub fn set_allowed_storage_classes(mut self, allowed_storage_classes: Vec<String>) -> Self {
        self.allowed_storage_classes = Some(allowed_storage_classes);
        self
    }

    /// Get allowed_storage_classes
    pub fn allowed_storage_classes(&self) -> Option<&Vec<String>> {
        self.allowed_storage_classes.as_ref()
    }

    /// Set allowed_sync_modes
    pub fn set_allowed_sync_modes(mut self, allowed_sync_modes: Vec<String>) -> Self {
        self.allowed_sync_modes = Some(allowed_sync_modes);
        self
    }

    /// Get allowed_sync_modes
    pub fn allowed_sync_modes(&self) -> Option<&Vec<String>> {
        self.allowed_sync_modes.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_dedicated_database_limits_creation() {
        let _model = <BillingPlanDedicatedDatabaseLimits as Default>::default();
    }

    #[test]
    fn test_billing_plan_dedicated_database_limits_serialization() {
        let model = <BillingPlanDedicatedDatabaseLimits as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlanDedicatedDatabaseLimits, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
