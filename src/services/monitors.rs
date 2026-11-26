//! Monitors service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Monitors service watches your project's usage and raises an alert when
/// a monitor's rules hold, announced by email, in the console or to a webhook.
#[derive(Debug, Clone)]
pub struct Monitors {
    client: Client,
}

impl Monitors {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of the project's monitors of every type. Use queries to filter
    /// by type, resource, severity or status, and `Query.search('search', term)`
    /// to search monitor IDs, names, metrics and resources.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MonitorList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a DNS monitor. It asks a resolver for a domain's records of one type
    /// on its own interval, and opens an alert when the resolver fails, or when
    /// its rules match the answers, by default when there is no such record. Each
    /// alert fires the `monitors.[monitorId].alerts.[alertId].create` event when
    /// it opens and `.update` when a check passes again, for project webhooks,
    /// functions and realtime, and notifies the organization's members by email
    /// and in the console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_dns_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: impl Into<String>,
        domain: impl Into<String>,
        record: Option<crate::enums::MonitorRecord>,
        resolver: Option<&str>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        params.insert("monitorId".to_string(), json!(monitor_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("domain".to_string(), json!(domain.into()));
        if let Some(value) = record {
            params.insert("record".to_string(), json!(value));
        }
        if let Some(value) = resolver {
            params.insert("resolver".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors/dns".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a DNS monitor by its unique ID. Only the parameters passed change;
    /// the monitor's state, its last check and its open alert are kept. Passing
    /// `rules` replaces all of them.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_dns_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: Option<&str>,
        domain: Option<&str>,
        record: Option<crate::enums::MonitorRecord>,
        resolver: Option<&str>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = record {
            params.insert("record".to_string(), json!(value));
        }
        if let Some(value) = resolver {
            params.insert("resolver".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/dns/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create an HTTP monitor. It requests a URL on its own interval, following
    /// redirects, and opens an alert when the request fails or when its rules
    /// match the response, by default a status outside 2xx or the certificate of
    /// an HTTPS URL about to expire. The response body is discarded as it arrives.
    /// Each alert fires the `monitors.[monitorId].alerts.[alertId].create` event
    /// when it opens and `.update` when a check passes again, for project
    /// webhooks, functions and realtime, and notifies the organization's members
    /// by email and in the console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_http_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: impl Into<String>,
        url: impl Into<String>,
        method: Option<crate::enums::MonitorMethod>,
        headers: Option<serde_json::Value>,
        body: Option<&str>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        params.insert("monitorId".to_string(), json!(monitor_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("url".to_string(), json!(url.into()));
        if let Some(value) = method {
            params.insert("method".to_string(), json!(value));
        }
        if let Some(value) = headers {
            params.insert("headers".to_string(), json!(value));
        }
        if let Some(value) = body {
            params.insert("body".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors/http".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an HTTP monitor by its unique ID. Only the parameters passed change;
    /// the monitor's state, its last check and its open alert are kept. Passing
    /// `rules` replaces all of them.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_http_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: Option<&str>,
        url: Option<&str>,
        method: Option<crate::enums::MonitorMethod>,
        headers: Option<serde_json::Value>,
        body: Option<&str>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = url {
            params.insert("url".to_string(), json!(value));
        }
        if let Some(value) = method {
            params.insert("method".to_string(), json!(value));
        }
        if let Some(value) = headers {
            params.insert("headers".to_string(), json!(value));
        }
        if let Some(value) = body {
            params.insert("body".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/http/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create an SMTP monitor. It reads a mail server's greeting, sends `EHLO`
    /// and, as its TLS mode asks, upgrades with `STARTTLS` or speaks TLS from the
    /// start, on its own interval. It opens an alert when the server cannot be
    /// reached, refuses the session or fails the TLS handshake, or when its rules
    /// match. It never authenticates or sends mail. Each alert fires the
    /// `monitors.[monitorId].alerts.[alertId].create` event when it opens and
    /// `.update` when a check passes again, for project webhooks, functions and
    /// realtime, and notifies the organization's members by email and in the
    /// console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_smtp_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: i64,
        tls_mode: Option<crate::enums::MonitorTlsMode>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        params.insert("monitorId".to_string(), json!(monitor_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("host".to_string(), json!(host.into()));
        params.insert("port".to_string(), json!(port));
        if let Some(value) = tls_mode {
            params.insert("tlsMode".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors/smtp".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an SMTP monitor by its unique ID. Only the parameters passed change;
    /// the monitor's state, its last check and its open alert are kept. Passing
    /// `rules` replaces all of them.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_smtp_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: Option<&str>,
        host: Option<&str>,
        port: Option<i64>,
        tls_mode: Option<crate::enums::MonitorTlsMode>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = host {
            params.insert("host".to_string(), json!(value));
        }
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = tls_mode {
            params.insert("tlsMode".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/smtp/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a TCP monitor. It opens a connection to a host and port on its own
    /// interval and closes it at once, and opens an alert when the connection
    /// cannot be made, or when its rules match. Each alert fires the
    /// `monitors.[monitorId].alerts.[alertId].create` event when it opens and
    /// `.update` when a check passes again, for project webhooks, functions and
    /// realtime, and notifies the organization's members by email and in the
    /// console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_tcp_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: i64,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        params.insert("monitorId".to_string(), json!(monitor_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("host".to_string(), json!(host.into()));
        params.insert("port".to_string(), json!(port));
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors/tcp".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a TCP monitor by its unique ID. Only the parameters passed change;
    /// the monitor's state, its last check and its open alert are kept. Passing
    /// `rules` replaces all of them.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_tcp_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: Option<&str>,
        host: Option<&str>,
        port: Option<i64>,
        rules: Option<Vec<String>>,
        interval: Option<i64>,
        timeout: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = host {
            params.insert("host".to_string(), json!(value));
        }
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = timeout {
            params.insert("timeout".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/tcp/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a usage monitor. It reads a usage metric of the project, or of one
    /// of its databases, buckets, functions or sites, on a schedule, and opens an
    /// alert when all its rules hold. Each alert fires the
    /// `monitors.[monitorId].alerts.[alertId].create` event when it opens and
    /// `.update` when usage recovers, for project webhooks, functions and
    /// realtime, and notifies the confirmed members of the `notify` roles by email
    /// and in the console, or the steps of the escalation set by `escalationId`.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_usage_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: impl Into<String>,
        rules: impl IntoIterator<Item = impl Into<String>>,
        window: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        params.insert("monitorId".to_string(), json!(monitor_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert(
            "rules".to_string(),
            json!(rules.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
        );
        if let Some(value) = window {
            params.insert("window".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/monitors/usage".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a usage monitor by its unique ID. Only the parameters passed change;
    /// the monitor's state and its open alert are kept. Passing `rules` replaces
    /// all of them.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_usage_monitor(
        &self,
        monitor_id: impl Into<String>,
        name: Option<&str>,
        rules: Option<Vec<String>>,
        window: Option<i64>,
        cooldown: Option<i64>,
        enabled: Option<bool>,
        notify: Option<Vec<String>>,
        severity: Option<crate::enums::MonitorSeverity>,
        escalation_id: Option<&str>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = rules {
            params.insert(
                "rules".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = window {
            params.insert("window".to_string(), json!(value));
        }
        if let Some(value) = cooldown {
            params.insert("cooldown".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = notify {
            params.insert(
                "notify".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = severity {
            params.insert("severity".to_string(), json!(value));
        }
        if let Some(value) = escalation_id {
            params.insert("escalationId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/usage/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a monitor by its unique ID, with its state as of its last evaluation.
    pub async fn get(
        &self,
        monitor_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Monitor> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a monitor by its unique ID. It stops being evaluated; an alert it is
    /// firing is resolved and kept with the rest of its alert history.
    pub async fn delete(&self, monitor_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let monitor_id = monitor_id.into();
        if monitor_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"monitorId\"",
                None,
                String::new(),
            ));
        }

        let path = "/monitors/{monitorId}"
            .to_string()
            .replace("{monitorId}", &Client::encode_path(&monitor_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Monitors {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitors_creation() {
        let client = Client::new();
        let service = Monitors::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
