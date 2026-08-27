//! Mysql service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Mysql {
    client: Client,
}

impl Mysql {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all dedicated databases. Results support pagination.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new dedicated database with the chosen engine and configuration.
    /// Status will be 'provisioning' until the database is ready.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        database_id: impl Into<String>,
        name: impl Into<String>,
        version: Option<&str>,
        specification: Option<&str>,
        replicas: Option<i64>,
        sync_mode: Option<&str>,
        network_idle_timeout_seconds: Option<i64>,
        network_ip_allowlist: Option<Vec<String>>,
        idle_timeout_minutes: Option<i64>,
        pitr: Option<bool>,
        pitr_retention_days: Option<i64>,
        storage_autoscaling: Option<bool>,
        storage_autoscaling_threshold_percent: Option<i64>,
        storage_autoscaling_max_gb: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        params.insert("databaseId".to_string(), json!(database_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = version {
            params.insert("version".to_string(), json!(value));
        }
        if let Some(value) = specification {
            params.insert("specification".to_string(), json!(value));
        }
        if let Some(value) = replicas {
            params.insert("replicas".to_string(), json!(value));
        }
        if let Some(value) = sync_mode {
            params.insert("syncMode".to_string(), json!(value));
        }
        if let Some(value) = network_idle_timeout_seconds {
            params.insert("networkIdleTimeoutSeconds".to_string(), json!(value));
        }
        if let Some(value) = network_ip_allowlist {
            params.insert(
                "networkIPAllowlist".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = idle_timeout_minutes {
            params.insert("idleTimeoutMinutes".to_string(), json!(value));
        }
        if let Some(value) = pitr {
            params.insert("pitr".to_string(), json!(value));
        }
        if let Some(value) = pitr_retention_days {
            params.insert("pitrRetentionDays".to_string(), json!(value));
        }
        if let Some(value) = storage_autoscaling {
            params.insert("storageAutoscaling".to_string(), json!(value));
        }
        if let Some(value) = storage_autoscaling_threshold_percent {
            params.insert(
                "storageAutoscalingThresholdPercent".to_string(),
                json!(value),
            );
        }
        if let Some(value) = storage_autoscaling_max_gb {
            params.insert("storageAutoscalingMaxGb".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the dedicated database specifications available on the current plan.
    /// Each specification reports its resource limits, pricing, and whether it is
    /// enabled for the organization.
    pub async fn list_specifications(
        &self,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseSpecificationList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/specifications".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a dedicated database by its unique ID. Returns the database
    /// configuration and current status.
    pub async fn get(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a dedicated database configuration. All changes are applied with
    /// zero downtime. Specification changes (cpu, memory, storage) are handled via
    /// rolling cutover. Storage expansion is done online. All other settings are
    /// applied in-place.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        database_id: impl Into<String>,
        name: Option<&str>,
        status: Option<&str>,
        specification: Option<&str>,
        replicas: Option<i64>,
        sync_mode: Option<&str>,
        network_idle_timeout_seconds: Option<i64>,
        network_ip_allowlist: Option<Vec<String>>,
        idle_timeout_minutes: Option<i64>,
        pitr: Option<bool>,
        pitr_retention_days: Option<i64>,
        storage_autoscaling: Option<bool>,
        storage_autoscaling_threshold_percent: Option<i64>,
        storage_autoscaling_max_gb: Option<i64>,
        metrics_trace_sample_rate: Option<f64>,
        metrics_slow_query_log_threshold_ms: Option<i64>,
        sql_api_enabled: Option<bool>,
        sql_api_allowed_statements: Option<Vec<String>>,
        sql_api_max_rows: Option<i64>,
        sql_api_max_bytes: Option<i64>,
        sql_api_timeout_seconds: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = status {
            params.insert("status".to_string(), json!(value));
        }
        if let Some(value) = specification {
            params.insert("specification".to_string(), json!(value));
        }
        if let Some(value) = replicas {
            params.insert("replicas".to_string(), json!(value));
        }
        if let Some(value) = sync_mode {
            params.insert("syncMode".to_string(), json!(value));
        }
        if let Some(value) = network_idle_timeout_seconds {
            params.insert("networkIdleTimeoutSeconds".to_string(), json!(value));
        }
        if let Some(value) = network_ip_allowlist {
            params.insert(
                "networkIPAllowlist".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = idle_timeout_minutes {
            params.insert("idleTimeoutMinutes".to_string(), json!(value));
        }
        if let Some(value) = pitr {
            params.insert("pitr".to_string(), json!(value));
        }
        if let Some(value) = pitr_retention_days {
            params.insert("pitrRetentionDays".to_string(), json!(value));
        }
        if let Some(value) = storage_autoscaling {
            params.insert("storageAutoscaling".to_string(), json!(value));
        }
        if let Some(value) = storage_autoscaling_threshold_percent {
            params.insert(
                "storageAutoscalingThresholdPercent".to_string(),
                json!(value),
            );
        }
        if let Some(value) = storage_autoscaling_max_gb {
            params.insert("storageAutoscalingMaxGb".to_string(), json!(value));
        }
        if let Some(value) = metrics_trace_sample_rate {
            params.insert("metricsTraceSampleRate".to_string(), json!(value));
        }
        if let Some(value) = metrics_slow_query_log_threshold_ms {
            params.insert("metricsSlowQueryLogThresholdMs".to_string(), json!(value));
        }
        if let Some(value) = sql_api_enabled {
            params.insert("sqlApiEnabled".to_string(), json!(value));
        }
        if let Some(value) = sql_api_allowed_statements {
            params.insert(
                "sqlApiAllowedStatements".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = sql_api_max_rows {
            params.insert("sqlApiMaxRows".to_string(), json!(value));
        }
        if let Some(value) = sql_api_max_bytes {
            params.insert("sqlApiMaxBytes".to_string(), json!(value));
        }
        if let Some(value) = sql_api_timeout_seconds {
            params.insert("sqlApiTimeoutSeconds".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a dedicated database. This action is irreversible. The database
    /// status will be set to 'deleting' and all resources will be cleaned up.
    /// Deletion is allowed from any state, and repeating the call re-dispatches
    /// the cleanup.
    pub async fn delete(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// List all backups for a dedicated database. Results can be filtered by
    /// status and type.
    pub async fn list_backups(
        &self,
        database_id: impl Into<String>,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseBackupList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a manual backup of a dedicated database. The backup will be created
    /// asynchronously and its status can be checked via the get backup endpoint.
    pub async fn create_backup(
        &self,
        database_id: impl Into<String>,
        r#type: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseBackup> {
        let mut params = HashMap::new();
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List scheduled backup policies for a dedicated database.
    pub async fn list_backup_policies(
        &self,
        database_id: impl Into<String>,
        queries: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::BackupPolicyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/policies"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a scheduled backup policy for a dedicated database.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_backup_policy(
        &self,
        database_id: impl Into<String>,
        policy_id: impl Into<String>,
        name: impl Into<String>,
        schedule: impl Into<String>,
        retention: i64,
        r#type: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let mut params = HashMap::new();
        params.insert("policyId".to_string(), json!(policy_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("schedule".to_string(), json!(schedule.into()));
        params.insert("retention".to_string(), json!(retention));
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/policies"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a scheduled backup policy for a dedicated database.
    pub async fn get_backup_policy(
        &self,
        database_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/policies/{policyId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{policyId}", &policy_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a scheduled backup policy for a dedicated database.
    pub async fn update_backup_policy(
        &self,
        database_id: impl Into<String>,
        policy_id: impl Into<String>,
        name: Option<&str>,
        schedule: Option<&str>,
        retention: Option<i64>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::BackupPolicy> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = schedule {
            params.insert("schedule".to_string(), json!(value));
        }
        if let Some(value) = retention {
            params.insert("retention".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/policies/{policyId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{policyId}", &policy_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a scheduled backup policy for a dedicated database. Backups already
    /// taken by the policy are kept until their retention expires.
    pub async fn delete_backup_policy(
        &self,
        database_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/policies/{policyId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{policyId}", &policy_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Configure off-cluster backup storage for a dedicated database. Supports S3,
    /// GCS, and Azure Blob Storage destinations. Backups will be stored to the
    /// configured destination in addition to on-cluster storage.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_backup_storage(
        &self,
        database_id: impl Into<String>,
        provider: impl Into<String>,
        bucket: impl Into<String>,
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
        region: Option<&str>,
        prefix: Option<&str>,
        endpoint: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseBackupStorage> {
        let mut params = HashMap::new();
        params.insert("provider".to_string(), json!(provider.into()));
        params.insert("bucket".to_string(), json!(bucket.into()));
        if let Some(value) = region {
            params.insert("region".to_string(), json!(value));
        }
        if let Some(value) = prefix {
            params.insert("prefix".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        params.insert("accessKey".to_string(), json!(access_key.into()));
        params.insert("secretKey".to_string(), json!(secret_key.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/storage"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get details of a specific database backup including its status, size, and
    /// timestamps.
    pub async fn get_backup(
        &self,
        database_id: impl Into<String>,
        backup_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseBackup> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/{backupId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{backupId}", &backup_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a database backup. This will permanently remove the backup from
    /// storage and cannot be undone.
    pub async fn delete_backup(
        &self,
        database_id: impl Into<String>,
        backup_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/backups/{backupId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{backupId}", &backup_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// List all ephemeral branches for a dedicated database. Returns branch
    /// metadata including ID, name, namespace, and expiration time.
    pub async fn list_branches(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseBranchList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/branches"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create an ephemeral database branch from the primary via PVC snapshot. The
    /// branch is a full copy of the database at the current point in time, useful
    /// for testing schema migrations or running experiments without affecting
    /// production data. Branches expire after the configured TTL (default 24
    /// hours). The branch is created asynchronously.
    pub async fn create_branch(
        &self,
        database_id: impl Into<String>,
        branch_id: Option<&str>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        if let Some(value) = branch_id {
            params.insert("branchId".to_string(), json!(value));
        }
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/branches"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete an ephemeral database branch. This removes the branch namespace, its
    /// PVC, and the associated VolumeSnapshot. The deletion runs asynchronously
    /// and is irreversible.
    pub async fn delete_branch(
        &self,
        database_id: impl Into<String>,
        branch_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/branches/{branchId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{branchId}", &branch_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Rotate the primary connection credentials for a dedicated database.
    /// Generates a new password and updates the database atomically. Previous
    /// credentials stop working immediately. Returns the database with a refreshed
    /// connection string carrying the new password.
    pub async fn update_credentials(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/credentials"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Execute SQL through the console-facing Cloud endpoint. Cloud proxies
    /// through the edge platform to the per-database SQL API sidecar. Application
    /// traffic should bypass cloud entirely and POST directly to the per-database
    /// hostname:
    /// `https://db-{project}-{db}.{region}.appwrite.center/v1/sql/executions` with
    /// an `X-Appwrite-Key` header — that path scales to the whole DB fleet
    /// without a per-query cloud round-trip. The statement type must be on the
    /// database's configured allow-list. Use bound parameters for any
    /// user-supplied values — the API does not interpolate raw strings.
    pub async fn create_execution(
        &self,
        database_id: impl Into<String>,
        sql: impl Into<String>,
        bindings: Option<serde_json::Value>,
        timeout_seconds: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseExecution> {
        let mut params = HashMap::new();
        params.insert("sql".to_string(), json!(sql.into()));
        if let Some(value) = bindings {
            params.insert("bindings".to_string(), json!(value));
        }
        if let Some(value) = timeout_seconds {
            params.insert("timeoutSeconds".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/executions"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Trigger a manual failover for a dedicated database with high availability
    /// enabled. Promotes a replica to primary. The failover runs asynchronously;
    /// poll the database document for status updates. A database left
    /// mid-operation also accepts this call as a repair once nothing is driving
    /// the operation it is stuck in. Repairing a failover that did not finish, a
    /// `failed` database, a stranded upgrade or migrate, or a stranded compute
    /// resize additionally requires `targetReplicaId` to name the member to
    /// promote, because the default target may be the member that operation
    /// already promoted.
    pub async fn create_failover(
        &self,
        database_id: impl Into<String>,
        target_replica_id: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        if let Some(value) = target_replica_id {
            params.insert("targetReplicaId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/failovers"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update the maintenance window for a dedicated database. Maintenance
    /// operations like minor version upgrades will be performed during this
    /// window.
    pub async fn update_maintenance(
        &self,
        database_id: impl Into<String>,
        day: impl Into<String>,
        hour_utc: i64,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        params.insert("day".to_string(), json!(day.into()));
        params.insert("hourUtc".to_string(), json!(hour_utc));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/maintenance"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Migrate a database between shared and dedicated types. Shared to dedicated
    /// provisions an always-on dedicated instance; dedicated to shared converts to
    /// a serverless instance that scales to zero when idle. Data is copied to the
    /// target with a brief read-only window during cutover.
    pub async fn create_migration(
        &self,
        database_id: impl Into<String>,
        target_type: impl Into<String>,
        specification: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        params.insert("targetType".to_string(), json!(target_type.into()));
        if let Some(value) = specification {
            params.insert("specification".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/migrations"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the lifecycle operations recorded for a dedicated database, newest
    /// first. Every provision, update, restore, backup and replication action is
    /// recorded here with its outcome, including an attempt that was abandoned
    /// because another worker took over the database.
    pub async fn list_operations(
        &self,
        database_id: impl Into<String>,
        status: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseOperationList> {
        let mut params = HashMap::new();
        if let Some(value) = status {
            params.insert("status".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/operations"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get available point-in-time recovery windows for a dedicated database.
    /// Returns the earliest and latest recovery points.
    pub async fn get_pitr(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabasePITRWindows> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/pitr"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the connection pooler configuration for a dedicated database. Returns
    /// pooler mode, max connections, and pool size settings.
    pub async fn get_pooler(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabasePooler> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/pooler"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update the connection pooler configuration for a dedicated database.
    /// Configure pool mode, max connections, and pool sizes.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_pooler(
        &self,
        database_id: impl Into<String>,
        mode: Option<&str>,
        max_connections: Option<i64>,
        default_pool_size: Option<i64>,
        read_write_splitting: Option<bool>,
        pooler_cpu_request: Option<&str>,
        pooler_cpu_limit: Option<&str>,
        pooler_memory_request: Option<&str>,
        pooler_memory_limit: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabasePooler> {
        let mut params = HashMap::new();
        if let Some(value) = mode {
            params.insert("mode".to_string(), json!(value));
        }
        if let Some(value) = max_connections {
            params.insert("maxConnections".to_string(), json!(value));
        }
        if let Some(value) = default_pool_size {
            params.insert("defaultPoolSize".to_string(), json!(value));
        }
        if let Some(value) = read_write_splitting {
            params.insert("readWriteSplitting".to_string(), json!(value));
        }
        if let Some(value) = pooler_cpu_request {
            params.insert("poolerCpuRequest".to_string(), json!(value));
        }
        if let Some(value) = pooler_cpu_limit {
            params.insert("poolerCpuLimit".to_string(), json!(value));
        }
        if let Some(value) = pooler_memory_request {
            params.insert("poolerMemoryRequest".to_string(), json!(value));
        }
        if let Some(value) = pooler_memory_limit {
            params.insert("poolerMemoryLimit".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/pooler"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get high availability status for a dedicated database. Returns replica
    /// statuses, replication lag, and sync mode.
    pub async fn get_replicas(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseReplicas> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/replicas"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// List all restorations for a dedicated database. Results can be filtered by
    /// status and type.
    pub async fn list_restorations(
        &self,
        database_id: impl Into<String>,
        status: Option<&str>,
        r#type: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseRestorationList> {
        let mut params = HashMap::new();
        if let Some(value) = status {
            params.insert("status".to_string(), json!(value));
        }
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/restorations"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Restore a database from a backup or to a specific point in time (PITR). For
    /// backup restoration, provide a backupId. For PITR, provide a targetTime as
    /// an ISO 8601 datetime. PITR requires the database to have PITR enabled and
    /// is only available for enterprise databases.
    pub async fn create_restoration(
        &self,
        database_id: impl Into<String>,
        r#type: Option<&str>,
        backup_id: Option<&str>,
        target_database_id: Option<&str>,
        target_time: Option<&str>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseRestoration> {
        let mut params = HashMap::new();
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        if let Some(value) = backup_id {
            params.insert("backupId".to_string(), json!(value));
        }
        if let Some(value) = target_database_id {
            params.insert("targetDatabaseId".to_string(), json!(value));
        }
        if let Some(value) = target_time {
            params.insert("targetTime".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/restorations"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get details of a specific database restoration including its status, type,
    /// and timestamps.
    pub async fn get_restoration(
        &self,
        database_id: impl Into<String>,
        restoration_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabaseRestoration> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/restorations/{restorationId}"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string())
            .replace("{restorationId}", &restoration_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get real-time health and status information for a dedicated database.
    /// Returns health status, readiness, uptime, connection info, replica status,
    /// and volume information.
    pub async fn get_status(
        &self,
        database_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DatabaseStatus> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/status"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Upgrade a dedicated database to a new engine version. Uses blue-green
    /// deployment for zero-downtime cutover.
    pub async fn create_upgrade(
        &self,
        database_id: impl Into<String>,
        target_version: impl Into<String>,
    ) -> crate::error::Result<crate::models::DedicatedDatabase> {
        let mut params = HashMap::new();
        params.insert("targetVersion".to_string(), json!(target_version.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/mysql/{databaseId}/upgrades"
            .to_string()
            .replace("{databaseId}", &database_id.into().to_string());

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Mysql {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mysql_creation() {
        let client = Client::new();
        let service = Mysql::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
