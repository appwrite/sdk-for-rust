//! Health service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Health service allows you to both validate and monitor your Appwrite
/// server's health.
#[derive(Debug, Clone)]
pub struct Health {
    client: Client,
}

impl Health {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Check the Appwrite HTTP server is up and responsive.
    pub async fn get(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatus> {
        let params = HashMap::new();

        let path = "/health".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite Antivirus server is up and connection is successful.
    pub async fn get_antivirus(
        &self,
    ) -> crate::error::Result<crate::models::HealthAntivirus> {
        let params = HashMap::new();

        let path = "/health/anti-virus".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite in-memory cache servers are up and connection is
    /// successful.
    pub async fn get_cache(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatusList> {
        let params = HashMap::new();

        let path = "/health/cache".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the SSL certificate for a domain
    pub async fn get_certificate(
        &self,
        domain: Option<&str>,
    ) -> crate::error::Result<crate::models::HealthCertificate> {
        let mut params = HashMap::new();
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }

        let path = "/health/certificate".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get console pausing health status. Monitors projects approaching the pause
    /// threshold to detect potential issues with console access tracking.
    pub async fn get_console_pausing(
        &self,
        threshold: Option<i64>,
        inactivity_days: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthStatus> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }
        if let Some(value) = inactivity_days {
            params.insert("inactivityDays".to_string(), json!(value));
        }

        let path = "/health/console-pausing".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite database servers are up and connection is successful.
    pub async fn get_db(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatusList> {
        let params = HashMap::new();

        let path = "/health/db".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite pub-sub servers are up and connection is successful.
    pub async fn get_pub_sub(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatusList> {
        let params = HashMap::new();

        let path = "/health/pubsub".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of audit logs that are waiting to be processed in the
    /// Appwrite internal queue server.
    pub async fn get_queue_audits(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/audits".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get billing project aggregation queue.
    pub async fn get_queue_billing_project_aggregation(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/billing-project-aggregation".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get billing team aggregation queue.
    pub async fn get_queue_billing_team_aggregation(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/billing-team-aggregation".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of builds that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_builds(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/builds".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the priority builds queue size.
    pub async fn get_queue_priority_builds(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/builds-priority".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of certificates that are waiting to be issued against
    /// [Letsencrypt](https://letsencrypt.org/) in the Appwrite internal queue
    /// server.
    pub async fn get_queue_certificates(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/certificates".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of database changes that are waiting to be processed in the
    /// Appwrite internal queue server.
    pub async fn get_queue_databases(
        &self,
        name: Option<&str>,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/databases".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of background destructive changes that are waiting to be
    /// processed in the Appwrite internal queue server.
    pub async fn get_queue_deletes(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/deletes".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Returns the amount of failed jobs in a given queue.
    pub async fn get_failed_jobs(
        &self,
        name: crate::enums::Name,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/failed/{name}".to_string().replace("{name}", &name.to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of function executions that are waiting to be processed in
    /// the Appwrite internal queue server.
    pub async fn get_queue_functions(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/functions".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of logs that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_logs(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/logs".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of mails that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_mails(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/mails".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of messages that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_messaging(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/messaging".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of migrations that are waiting to be processed in the
    /// Appwrite internal queue server.
    pub async fn get_queue_migrations(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/migrations".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get region manager queue.
    pub async fn get_queue_region_manager(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/region-manager".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of metrics that are waiting to be processed in the Appwrite
    /// stats resources queue.
    pub async fn get_queue_stats_resources(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/stats-resources".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of metrics that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_usage(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/stats-usage".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get threats queue.
    pub async fn get_queue_threats(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/threats".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the number of webhooks that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub async fn get_queue_webhooks(
        &self,
        threshold: Option<i64>,
    ) -> crate::error::Result<crate::models::HealthQueue> {
        let mut params = HashMap::new();
        if let Some(value) = threshold {
            params.insert("threshold".to_string(), json!(value));
        }

        let path = "/health/queue/webhooks".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite storage device is up and connection is successful.
    pub async fn get_storage(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatus> {
        let params = HashMap::new();

        let path = "/health/storage".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite local storage device is up and connection is successful.
    pub async fn get_storage_local(
        &self,
    ) -> crate::error::Result<crate::models::HealthStatus> {
        let params = HashMap::new();

        let path = "/health/storage/local".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Check the Appwrite server time is synced with Google remote NTP server. We
    /// use this technology to smoothly handle leap seconds with no disruptive
    /// events. The [Network Time
    /// Protocol](https://en.wikipedia.org/wiki/Network_Time_Protocol) (NTP) is
    /// used by hundreds of millions of computers and devices to synchronize their
    /// clocks over the Internet. If your computer sets its own clock, it likely
    /// uses NTP.
    pub async fn get_time(
        &self,
    ) -> crate::error::Result<crate::models::HealthTime> {
        let params = HashMap::new();

        let path = "/health/time".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Health {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_creation() {
        let client = Client::new();
        let service = Health::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
