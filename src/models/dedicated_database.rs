//! DedicatedDatabase model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// DedicatedDatabase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabase {
    /// Dedicated database ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Database creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Project ID that owns this database.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Database display name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Product API that owns this database: tablesdb, documentsdb, vectorsdb,
    /// mysql, postgresql, or mongodb.
    #[serde(rename = "api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    /// Database engine: postgresql, mysql, or mongodb. Null until the backing
    /// reports one.
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Database engine version.
    #[serde(rename = "version")]
    #[serde(default = "default_version")]
    pub version: String,
    /// Specification identifier.
    #[serde(rename = "specification")]
    #[serde(default = "default_specification")]
    pub specification: String,
    /// Database backend provider. Possible values: edge.
    #[serde(rename = "backend")]
    #[serde(default = "default_backend")]
    pub backend: String,
    /// Database hostname for connections.
    #[serde(rename = "hostname")]
    #[serde(default = "default_hostname")]
    pub hostname: String,
    /// Database port for connections. Derived from the engine when the backing has
    /// not reported one yet.
    #[serde(rename = "connectionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_port: Option<i64>,
    /// Database username for connections.
    #[serde(rename = "connectionUser")]
    #[serde(default = "default_connection_user")]
    pub connection_user: String,
    /// Database password for connections.
    #[serde(rename = "connectionPassword")]
    #[serde(default = "default_connection_password")]
    pub connection_password: String,
    /// Committed generation of the primary connection credentials. Null until the
    /// rotation contract has been initialized.
    #[serde(rename = "credentialGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_generation: Option<i64>,
    /// Full database connection string (URI format).
    #[serde(rename = "connectionString")]
    #[serde(default = "default_connection_string")]
    pub connection_string: String,
    /// Whether SSL/TLS is required for client connections.
    #[serde(rename = "ssl")]
    #[serde(default = "default_ssl")]
    pub ssl: bool,
    /// Database status. Possible values: provisioning, ready, inactive, paused,
    /// failed, deleted, restoring, scaling.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
    /// Container status for lifecycle-managed database runtimes: active, inactive,
    /// or failed (a wake was declined because the backing namespace is gone).
    #[serde(rename = "containerStatus")]
    #[serde(default = "default_container_status")]
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
    #[serde(default = "default_lifecycle_state")]
    pub lifecycle_state: String,
    /// Minutes of inactivity before container scales to zero.
    #[serde(rename = "idleTimeoutMinutes")]
    #[serde(default = "default_idle_timeout_minutes")]
    pub idle_timeout_minutes: i64,
    /// CPU allocated in millicores.
    #[serde(rename = "cpu")]
    #[serde(default = "default_cpu")]
    pub cpu: i64,
    /// Memory allocated in MB.
    #[serde(rename = "memory")]
    #[serde(default = "default_memory")]
    pub memory: i64,
    /// Storage allocated in GB.
    #[serde(rename = "storage")]
    #[serde(default = "default_storage")]
    pub storage: i64,
    /// Storage resize status. Possible values: idle (no resize in flight),
    /// resizing (the volume is growing towards storageTargetGb).
    #[serde(rename = "storageStatus")]
    #[serde(default = "default_storage_status")]
    pub storage_status: String,
    /// Size in GB the volume is growing towards while storageStatus is resizing. 0
    /// when no resize is in flight.
    #[serde(rename = "storageTargetGb")]
    #[serde(default = "default_storage_target_gb")]
    pub storage_target_gb: i64,
    /// Time the in-flight storage resize started, in ISO 8601 format.
    #[serde(rename = "storageResizeStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_resize_started_at: Option<String>,
    /// Storage class. Currently always 'ssd'; DigitalOcean exposes a single
    /// block-storage class.
    #[serde(rename = "storageClass")]
    #[serde(default = "default_storage_class")]
    pub storage_class: String,
    /// Maximum storage allowed in GB. 0 means use system default.
    #[serde(rename = "storageMaxGb")]
    #[serde(default = "default_storage_max_gb")]
    pub storage_max_gb: i64,
    /// Kubernetes node pool where the database is scheduled.
    #[serde(rename = "nodePool")]
    #[serde(default = "default_node_pool")]
    pub node_pool: String,
    /// Number of high availability replicas. High availability is enabled when
    /// greater than 0.
    #[serde(rename = "replicas")]
    #[serde(default = "default_replicas")]
    pub replicas: i64,
    /// Replication sync mode: async, sync, or quorum.
    #[serde(rename = "syncMode")]
    #[serde(default = "default_sync_mode")]
    pub sync_mode: String,
    /// Maximum concurrent client connections. This is the limit a client pool may
    /// reach; the engine's own max_connections reported by the status endpoint is
    /// a smaller backend limit the pooler multiplexes onto and does not constrain
    /// a client pool.
    #[serde(rename = "networkMaxConnections")]
    #[serde(default = "default_network_max_connections")]
    pub network_max_connections: i64,
    /// Connection idle timeout in seconds.
    #[serde(rename = "networkIdleTimeoutSeconds")]
    #[serde(default = "default_network_idle_timeout_seconds")]
    pub network_idle_timeout_seconds: i64,
    /// IP addresses/CIDR ranges allowed to connect.
    #[serde(rename = "networkIPAllowlist")]
    #[serde(default = "default_network_ip_allowlist")]
    pub network_ip_allowlist: Vec<String>,
    /// Whether automatic backups are enabled.
    #[serde(rename = "backupEnabled")]
    #[serde(default = "default_backup_enabled")]
    pub backup_enabled: bool,
    /// Whether point-in-time recovery is enabled.
    #[serde(rename = "pitr")]
    #[serde(default = "default_pitr")]
    pub pitr: bool,
    /// Number of days to retain PITR data.
    #[serde(rename = "pitrRetentionDays")]
    #[serde(default = "default_pitr_retention_days")]
    pub pitr_retention_days: i64,
    /// Whether automatic storage expansion is enabled.
    #[serde(rename = "storageAutoscaling")]
    #[serde(default = "default_storage_autoscaling")]
    pub storage_autoscaling: bool,
    /// Storage usage percentage that triggers automatic expansion.
    #[serde(rename = "storageAutoscalingThresholdPercent")]
    #[serde(default = "default_storage_autoscaling_threshold_percent")]
    pub storage_autoscaling_threshold_percent: i64,
    /// Maximum storage size in GB for autoscaling. Defaults to 3 times the
    /// specification's storage. 0 means no limit.
    #[serde(rename = "storageAutoscalingMaxGb")]
    #[serde(default = "default_storage_autoscaling_max_gb")]
    pub storage_autoscaling_max_gb: i64,
    /// Day of the week for the maintenance window. Possible values: sun, mon, tue,
    /// wed, thu, fri, sat.
    #[serde(rename = "maintenanceWindowDay")]
    #[serde(default = "default_maintenance_window_day")]
    pub maintenance_window_day: String,
    /// Hour in UTC (0-23) when the maintenance window starts.
    #[serde(rename = "maintenanceWindowHourUtc")]
    #[serde(default = "default_maintenance_window_hour_utc")]
    pub maintenance_window_hour_utc: i64,
    /// Whether metrics collection is enabled.
    #[serde(rename = "metricsEnabled")]
    #[serde(default = "default_metrics_enabled")]
    pub metrics_enabled: bool,
    /// Whether the SQL API sidecar is enabled for this database.
    #[serde(rename = "sqlApiEnabled")]
    #[serde(default = "default_sql_api_enabled")]
    pub sql_api_enabled: bool,
    /// Statement types accepted by the SQL API. Defaults to read/write DML only;
    /// DDL/DCL types (CREATE, ALTER, DROP, TRUNCATE, GRANT, REVOKE) are opt-in per
    /// database. Allowed values: SELECT, INSERT, UPDATE, DELETE, CREATE, ALTER,
    /// DROP, TRUNCATE, GRANT, REVOKE.
    #[serde(rename = "sqlApiAllowedStatements")]
    #[serde(default = "default_sql_api_allowed_statements")]
    pub sql_api_allowed_statements: Vec<String>,
    /// Maximum rows returned per SQL API execution. Results larger than this are
    /// truncated.
    #[serde(rename = "sqlApiMaxRows")]
    #[serde(default = "default_sql_api_max_rows")]
    pub sql_api_max_rows: i64,
    /// Maximum serialised SQL API result payload in bytes. Results larger than
    /// this are truncated.
    #[serde(rename = "sqlApiMaxBytes")]
    #[serde(default = "default_sql_api_max_bytes")]
    pub sql_api_max_bytes: i64,
    /// Maximum server-side SQL API execution time in seconds before the query is
    /// cancelled.
    #[serde(rename = "sqlApiTimeoutSeconds")]
    #[serde(default = "default_sql_api_timeout_seconds")]
    pub sql_api_timeout_seconds: i64,
    /// Error message if status is failed.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_project_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_version() -> String {
    "".to_string()
}

fn default_specification() -> String {
    "".to_string()
}

fn default_backend() -> String {
    "".to_string()
}

fn default_hostname() -> String {
    "".to_string()
}

fn default_connection_user() -> String {
    "".to_string()
}

fn default_connection_password() -> String {
    "".to_string()
}

fn default_connection_string() -> String {
    "".to_string()
}

fn default_ssl() -> bool {
    false
}

fn default_status() -> String {
    "provisioning".to_string()
}

fn default_container_status() -> String {
    "".to_string()
}

fn default_lifecycle_state() -> String {
    "".to_string()
}

fn default_idle_timeout_minutes() -> i64 {
    15
}

fn default_cpu() -> i64 {
    500
}

fn default_memory() -> i64 {
    512
}

fn default_storage() -> i64 {
    10
}

fn default_storage_status() -> String {
    "idle".to_string()
}

fn default_storage_target_gb() -> i64 {
    0
}

fn default_storage_class() -> String {
    "ssd".to_string()
}

fn default_storage_max_gb() -> i64 {
    0
}

fn default_node_pool() -> String {
    "".to_string()
}

fn default_replicas() -> i64 {
    0
}

fn default_sync_mode() -> String {
    "".to_string()
}

fn default_network_max_connections() -> i64 {
    100
}

fn default_network_idle_timeout_seconds() -> i64 {
    900
}

fn default_network_ip_allowlist() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_backup_enabled() -> bool {
    true
}

fn default_pitr() -> bool {
    true
}

fn default_pitr_retention_days() -> i64 {
    7
}

fn default_storage_autoscaling() -> bool {
    true
}

fn default_storage_autoscaling_threshold_percent() -> i64 {
    85
}

fn default_storage_autoscaling_max_gb() -> i64 {
    0
}

fn default_maintenance_window_day() -> String {
    "sun".to_string()
}

fn default_maintenance_window_hour_utc() -> i64 {
    3
}

fn default_metrics_enabled() -> bool {
    false
}

fn default_sql_api_enabled() -> bool {
    true
}

fn default_sql_api_allowed_statements() -> Vec<String> {
    serde_json::from_value(serde_json::json!(["SELECT", "INSERT", "UPDATE", "DELETE"])).unwrap()
}

fn default_sql_api_max_rows() -> i64 {
    10000
}

fn default_sql_api_max_bytes() -> i64 {
    10485760
}

fn default_sql_api_timeout_seconds() -> i64 {
    30
}

fn default_error() -> String {
    "".to_string()
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

    /// Set api
    pub fn set_api(mut self, api: String) -> Self {
        self.api = Some(api);
        self
    }

    /// Get api
    pub fn api(&self) -> Option<&String> {
        self.api.as_ref()
    }

    /// Set engine
    pub fn set_engine(mut self, engine: String) -> Self {
        self.engine = Some(engine);
        self
    }

    /// Get engine
    pub fn engine(&self) -> Option<&String> {
        self.engine.as_ref()
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

    /// Set connection_port
    pub fn set_connection_port(mut self, connection_port: i64) -> Self {
        self.connection_port = Some(connection_port);
        self
    }

    /// Get connection_port
    pub fn connection_port(&self) -> Option<&i64> {
        self.connection_port.as_ref()
    }

    /// Get connection_user
    pub fn connection_user(&self) -> &String {
        &self.connection_user
    }

    /// Get connection_password
    pub fn connection_password(&self) -> &String {
        &self.connection_password
    }

    /// Set credential_generation
    pub fn set_credential_generation(mut self, credential_generation: i64) -> Self {
        self.credential_generation = Some(credential_generation);
        self
    }

    /// Get credential_generation
    pub fn credential_generation(&self) -> Option<&i64> {
        self.credential_generation.as_ref()
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

    /// Get storage_status
    pub fn storage_status(&self) -> &String {
        &self.storage_status
    }

    /// Get storage_target_gb
    pub fn storage_target_gb(&self) -> &i64 {
        &self.storage_target_gb
    }

    /// Set storage_resize_started_at
    pub fn set_storage_resize_started_at(mut self, storage_resize_started_at: String) -> Self {
        self.storage_resize_started_at = Some(storage_resize_started_at);
        self
    }

    /// Get storage_resize_started_at
    pub fn storage_resize_started_at(&self) -> Option<&String> {
        self.storage_resize_started_at.as_ref()
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
        let _ = _model.version();
        let _ = _model.specification();
        let _ = _model.backend();
        let _ = _model.hostname();
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
        let _ = _model.storage_status();
        let _ = _model.storage_target_gb();
        let _ = _model.storage_class();
        let _ = _model.storage_max_gb();
        let _ = _model.node_pool();
        let _ = _model.replicas();
        let _ = _model.sync_mode();
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
