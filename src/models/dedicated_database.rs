//! DedicatedDatabase model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// DedicatedDatabase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabase {
    /// Dedicated database ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Database creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Project ID that owns this database.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Database display name.
    #[serde(rename = "name")]
    pub name: String,
    /// Product API that owns this database: tablesdb, documentsdb, vectorsdb,
    /// mysql, postgresql, or mongodb.
    #[serde(rename = "api")]
    pub api: String,
    /// Database engine: postgresql, mysql, mariadb, or mongodb.
    #[serde(rename = "engine")]
    pub engine: String,
    /// Database engine version.
    #[serde(rename = "version")]
    pub version: String,
    /// Specification identifier.
    #[serde(rename = "specification")]
    pub specification: String,
    /// Database backend provider. Possible values: prisma, edge.
    #[serde(rename = "backend")]
    pub backend: String,
    /// Database hostname for connections.
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// Database port for connections.
    #[serde(rename = "connectionPort")]
    pub connection_port: i64,
    /// Database username for connections.
    #[serde(rename = "connectionUser")]
    pub connection_user: String,
    /// Database password for connections.
    #[serde(rename = "connectionPassword")]
    pub connection_password: String,
    /// Full database connection string (URI format).
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    /// Whether SSL/TLS is required for client connections.
    #[serde(rename = "ssl")]
    pub ssl: bool,
    /// Database status. Possible values: provisioning, ready, inactive, paused,
    /// failed, deleted, restoring, scaling.
    #[serde(rename = "status")]
    pub status: String,
    /// Container status for lifecycle-managed database runtimes: active or
    /// inactive.
    #[serde(rename = "containerStatus")]
    pub container_status: String,
    /// Last activity timestamp in ISO 8601 format.
    #[serde(rename = "lastAccessedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    /// Display-only timestamp when the database is expected to be considered idle
    /// (ISO 8601 format). Derived from last activity; lifecycle transitions are
    /// driven by lifecycleState.
    #[serde(rename = "idleUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_until: Option<String>,
    /// Idle-lifecycle state of the database. Possible values: active, warm, cold,
    /// hibernated.
    #[serde(rename = "lifecycleState")]
    pub lifecycle_state: String,
    /// Minutes of inactivity before container scales to zero.
    #[serde(rename = "idleTimeoutMinutes")]
    pub idle_timeout_minutes: i64,
    /// CPU allocated in millicores.
    #[serde(rename = "cpu")]
    pub cpu: i64,
    /// Memory allocated in MB.
    #[serde(rename = "memory")]
    pub memory: i64,
    /// Storage allocated in GB.
    #[serde(rename = "storage")]
    pub storage: i64,
    /// Storage class. Currently always 'ssd'; DigitalOcean exposes a single
    /// block-storage class.
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    /// Maximum storage allowed in GB. 0 means use system default.
    #[serde(rename = "storageMaxGb")]
    pub storage_max_gb: i64,
    /// Kubernetes node pool where the database is scheduled.
    #[serde(rename = "nodePool")]
    pub node_pool: String,
    /// Number of high availability replicas. High availability is enabled when
    /// greater than 0.
    #[serde(rename = "replicas")]
    pub replicas: i64,
    /// Replication sync mode: async, sync, or quorum.
    #[serde(rename = "syncMode")]
    pub sync_mode: String,
    /// Number of cross-region replicas. Cross-region availability is enabled when
    /// greater than 0.
    #[serde(rename = "crossRegionReplicas")]
    pub cross_region_replicas: i64,
    /// Maximum concurrent connections.
    #[serde(rename = "networkMaxConnections")]
    pub network_max_connections: i64,
    /// Connection idle timeout in seconds.
    #[serde(rename = "networkIdleTimeoutSeconds")]
    pub network_idle_timeout_seconds: i64,
    /// IP addresses/CIDR ranges allowed to connect.
    #[serde(rename = "networkIPAllowlist")]
    pub network_ip_allowlist: Vec<String>,
    /// Whether automatic backups are enabled.
    #[serde(rename = "backupEnabled")]
    pub backup_enabled: bool,
    /// Whether point-in-time recovery is enabled.
    #[serde(rename = "pitr")]
    pub pitr: bool,
    /// Number of days to retain PITR data.
    #[serde(rename = "pitrRetentionDays")]
    pub pitr_retention_days: i64,
    /// Whether automatic storage expansion is enabled.
    #[serde(rename = "storageAutoscaling")]
    pub storage_autoscaling: bool,
    /// Storage usage percentage that triggers automatic expansion.
    #[serde(rename = "storageAutoscalingThresholdPercent")]
    pub storage_autoscaling_threshold_percent: i64,
    /// Maximum storage size in GB for autoscaling. 0 means no limit.
    #[serde(rename = "storageAutoscalingMaxGb")]
    pub storage_autoscaling_max_gb: i64,
    /// Day of the week for the maintenance window. Possible values: sun, mon, tue,
    /// wed, thu, fri, sat.
    #[serde(rename = "maintenanceWindowDay")]
    pub maintenance_window_day: String,
    /// Hour in UTC (0-23) when the maintenance window starts.
    #[serde(rename = "maintenanceWindowHourUtc")]
    pub maintenance_window_hour_utc: i64,
    /// Whether metrics collection is enabled.
    #[serde(rename = "metricsEnabled")]
    pub metrics_enabled: bool,
    /// Whether the SQL API sidecar is enabled for this database.
    #[serde(rename = "sqlApiEnabled")]
    pub sql_api_enabled: bool,
    /// Statement types accepted by the SQL API. Defaults to read/write DML only;
    /// DDL/DCL types (CREATE, ALTER, DROP, TRUNCATE, GRANT, REVOKE) are opt-in per
    /// database. Allowed values: SELECT, INSERT, UPDATE, DELETE, CREATE, ALTER,
    /// DROP, TRUNCATE, GRANT, REVOKE.
    #[serde(rename = "sqlApiAllowedStatements")]
    pub sql_api_allowed_statements: Vec<String>,
    /// Maximum rows returned per SQL API execution. Results larger than this are
    /// truncated.
    #[serde(rename = "sqlApiMaxRows")]
    pub sql_api_max_rows: i64,
    /// Maximum serialised SQL API result payload in bytes. Results larger than
    /// this are truncated.
    #[serde(rename = "sqlApiMaxBytes")]
    pub sql_api_max_bytes: i64,
    /// Maximum server-side SQL API execution time in seconds before the query is
    /// cancelled.
    #[serde(rename = "sqlApiTimeoutSeconds")]
    pub sql_api_timeout_seconds: i64,
    /// Error message if status is failed.
    #[serde(rename = "error")]
    pub error: String,
}

impl DedicatedDatabase {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get api
    pub fn api(&self) -> &String {
        &self.api
    }

    /// Get engine
    pub fn engine(&self) -> &String {
        &self.engine
    }

    /// Get version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get specification
    pub fn specification(&self) -> &String {
        &self.specification
    }

    /// Get backend
    pub fn backend(&self) -> &String {
        &self.backend
    }

    /// Get hostname
    pub fn hostname(&self) -> &String {
        &self.hostname
    }

    /// Get connection_port
    pub fn connection_port(&self) -> &i64 {
        &self.connection_port
    }

    /// Get connection_user
    pub fn connection_user(&self) -> &String {
        &self.connection_user
    }

    /// Get connection_password
    pub fn connection_password(&self) -> &String {
        &self.connection_password
    }

    /// Get connection_string
    pub fn connection_string(&self) -> &String {
        &self.connection_string
    }

    /// Get ssl
    pub fn ssl(&self) -> &bool {
        &self.ssl
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get container_status
    pub fn container_status(&self) -> &String {
        &self.container_status
    }

    /// Set last_accessed_at
    pub fn set_last_accessed_at(mut self, last_accessed_at: String) -> Self {
        self.last_accessed_at = Some(last_accessed_at);
        self
    }

    /// Get last_accessed_at
    pub fn last_accessed_at(&self) -> Option<&String> {
        self.last_accessed_at.as_ref()
    }

    /// Set idle_until
    pub fn set_idle_until(mut self, idle_until: String) -> Self {
        self.idle_until = Some(idle_until);
        self
    }

    /// Get idle_until
    pub fn idle_until(&self) -> Option<&String> {
        self.idle_until.as_ref()
    }

    /// Get lifecycle_state
    pub fn lifecycle_state(&self) -> &String {
        &self.lifecycle_state
    }

    /// Get idle_timeout_minutes
    pub fn idle_timeout_minutes(&self) -> &i64 {
        &self.idle_timeout_minutes
    }

    /// Get cpu
    pub fn cpu(&self) -> &i64 {
        &self.cpu
    }

    /// Get memory
    pub fn memory(&self) -> &i64 {
        &self.memory
    }

    /// Get storage
    pub fn storage(&self) -> &i64 {
        &self.storage
    }

    /// Get storage_class
    pub fn storage_class(&self) -> &String {
        &self.storage_class
    }

    /// Get storage_max_gb
    pub fn storage_max_gb(&self) -> &i64 {
        &self.storage_max_gb
    }

    /// Get node_pool
    pub fn node_pool(&self) -> &String {
        &self.node_pool
    }

    /// Get replicas
    pub fn replicas(&self) -> &i64 {
        &self.replicas
    }

    /// Get sync_mode
    pub fn sync_mode(&self) -> &String {
        &self.sync_mode
    }

    /// Get cross_region_replicas
    pub fn cross_region_replicas(&self) -> &i64 {
        &self.cross_region_replicas
    }

    /// Get network_max_connections
    pub fn network_max_connections(&self) -> &i64 {
        &self.network_max_connections
    }

    /// Get network_idle_timeout_seconds
    pub fn network_idle_timeout_seconds(&self) -> &i64 {
        &self.network_idle_timeout_seconds
    }

    /// Get network_ip_allowlist
    pub fn network_ip_allowlist(&self) -> &Vec<String> {
        &self.network_ip_allowlist
    }

    /// Get backup_enabled
    pub fn backup_enabled(&self) -> &bool {
        &self.backup_enabled
    }

    /// Get pitr
    pub fn pitr(&self) -> &bool {
        &self.pitr
    }

    /// Get pitr_retention_days
    pub fn pitr_retention_days(&self) -> &i64 {
        &self.pitr_retention_days
    }

    /// Get storage_autoscaling
    pub fn storage_autoscaling(&self) -> &bool {
        &self.storage_autoscaling
    }

    /// Get storage_autoscaling_threshold_percent
    pub fn storage_autoscaling_threshold_percent(&self) -> &i64 {
        &self.storage_autoscaling_threshold_percent
    }

    /// Get storage_autoscaling_max_gb
    pub fn storage_autoscaling_max_gb(&self) -> &i64 {
        &self.storage_autoscaling_max_gb
    }

    /// Get maintenance_window_day
    pub fn maintenance_window_day(&self) -> &String {
        &self.maintenance_window_day
    }

    /// Get maintenance_window_hour_utc
    pub fn maintenance_window_hour_utc(&self) -> &i64 {
        &self.maintenance_window_hour_utc
    }

    /// Get metrics_enabled
    pub fn metrics_enabled(&self) -> &bool {
        &self.metrics_enabled
    }

    /// Get sql_api_enabled
    pub fn sql_api_enabled(&self) -> &bool {
        &self.sql_api_enabled
    }

    /// Get sql_api_allowed_statements
    pub fn sql_api_allowed_statements(&self) -> &Vec<String> {
        &self.sql_api_allowed_statements
    }

    /// Get sql_api_max_rows
    pub fn sql_api_max_rows(&self) -> &i64 {
        &self.sql_api_max_rows
    }

    /// Get sql_api_max_bytes
    pub fn sql_api_max_bytes(&self) -> &i64 {
        &self.sql_api_max_bytes
    }

    /// Get sql_api_timeout_seconds
    pub fn sql_api_timeout_seconds(&self) -> &i64 {
        &self.sql_api_timeout_seconds
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_creation() {
        let _model = <DedicatedDatabase as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.project_id();
        let _ = _model.name();
        let _ = _model.api();
        let _ = _model.engine();
        let _ = _model.version();
        let _ = _model.specification();
        let _ = _model.backend();
        let _ = _model.hostname();
        let _ = _model.connection_port();
        let _ = _model.connection_user();
        let _ = _model.connection_password();
        let _ = _model.connection_string();
        let _ = _model.ssl();
        let _ = _model.status();
        let _ = _model.container_status();
        let _ = _model.lifecycle_state();
        let _ = _model.idle_timeout_minutes();
        let _ = _model.cpu();
        let _ = _model.memory();
        let _ = _model.storage();
        let _ = _model.storage_class();
        let _ = _model.storage_max_gb();
        let _ = _model.node_pool();
        let _ = _model.replicas();
        let _ = _model.sync_mode();
        let _ = _model.cross_region_replicas();
        let _ = _model.network_max_connections();
        let _ = _model.network_idle_timeout_seconds();
        let _ = _model.network_ip_allowlist();
        let _ = _model.backup_enabled();
        let _ = _model.pitr();
        let _ = _model.pitr_retention_days();
        let _ = _model.storage_autoscaling();
        let _ = _model.storage_autoscaling_threshold_percent();
        let _ = _model.storage_autoscaling_max_gb();
        let _ = _model.maintenance_window_day();
        let _ = _model.maintenance_window_hour_utc();
        let _ = _model.metrics_enabled();
        let _ = _model.sql_api_enabled();
        let _ = _model.sql_api_allowed_statements();
        let _ = _model.sql_api_max_rows();
        let _ = _model.sql_api_max_bytes();
        let _ = _model.sql_api_timeout_seconds();
        let _ = _model.error();
    }

    #[test]
    fn test_dedicated_database_serialization() {
        let model = <DedicatedDatabase as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabase, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
