//! Monitor model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Monitor {
    /// Monitor ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Monitor creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Monitor update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Monitor name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Project ID.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Rules that must all hold for the monitor to fire, as query strings. A usage
    /// monitor has one comparing a metric with a threshold, optionally scoped with
    /// `equal('resourceType')` and `equal('resourceId')`. A probe monitor fires
    /// when its rules match the check, over these variables: `http` `statusCode`,
    /// `latency`, `certificateDays` and `certificateLifetime`; `dns` `answer`,
    /// `answers` and `latency`; `tcp` `latency`; `smtp` `code`, `latency`,
    /// `certificateDays` and `certificateLifetime`. A failed connection fires
    /// whatever the rules.
    #[serde(rename = "rules")]
    #[serde(default = "default_rules")]
    pub rules: Vec<String>,
    /// What the monitor checks: `usage` compares a usage metric with a threshold;
    /// `http`, `dns`, `tcp` and `smtp` probe a target on their own interval.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: crate::enums::MonitorType,
    /// Type of the resource the monitor watches, from its rules: the project, or
    /// one database, bucket, function or site.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: crate::enums::MonitorResourceType,
    /// ID of the resource the monitor watches, from its rules. The project ID for
    /// a project-wide monitor.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Seconds a counter metric is summed over, and a growth metric is measured
    /// across. Gauges compare their latest value. Usage is read a short lag behind
    /// the evaluation (`_APP_ALERTS_USAGE_LAG`, 120 seconds by default) so events
    /// still being written are counted by the next evaluation.
    #[serde(rename = "window")]
    #[serde(default = "default_window")]
    pub window: i64,
    /// Minimum seconds between two team notifications of this monitor firing.
    /// Events fire on every change regardless.
    #[serde(rename = "cooldown")]
    #[serde(default = "default_cooldown")]
    pub cooldown: i64,
    /// Whether the monitor is evaluated.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Organization roles whose confirmed members get an email and a console
    /// notification when the monitor fires and resolves. Empty when the alert
    /// events are the only delivery. Not used while `escalationId` is set.
    #[serde(rename = "notify")]
    #[serde(default = "default_notify")]
    pub notify: Vec<String>,
    /// ID of the organization's escalation that notifies the team of this
    /// monitor's alerts, step by step until one is acknowledged. When set,
    /// `notify` is not used. Empty to notify the `notify` roles once.
    #[serde(rename = "escalationId")]
    #[serde(default = "default_escalation_id")]
    pub escalation_id: String,
    /// How urgent the monitor's alerts are: `info`, `warning`, `error` or
    /// `critical`. Critical alerts may be delivered with higher urgency by the
    /// notification channels that support it.
    #[serde(rename = "severity")]
    #[serde(default = "default_severity")]
    pub severity: crate::enums::MonitorSeverity,
    /// Whether the monitor is firing, as of its last evaluation.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: crate::enums::MonitorStatus,
    /// Metric value at the last evaluation. Null before the first evaluation, and
    /// when the last evaluation (`evaluatedAt`) found no usage recorded to read:
    /// without data a monitor is not compared, so it keeps its status and open
    /// alert.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Last evaluation date in ISO 8601 format.
    #[serde(rename = "evaluatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_at: Option<String>,
    /// ID of the alert the monitor is firing for. Empty while it is not firing.
    #[serde(rename = "alertId")]
    #[serde(default = "default_alert_id")]
    pub alert_id: String,
    /// Date the monitor last announced an alert, in ISO 8601 format.
    #[serde(rename = "triggeredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_at: Option<String>,
    /// Probe monitors: seconds between two checks, read back raised to the plan's
    /// minimum. 0 for usage monitors, which are evaluated on the server's
    /// interval.
    #[serde(rename = "interval")]
    #[serde(default = "default_interval")]
    pub interval: i64,
    /// Probe monitors: seconds a check may take before it fails.
    #[serde(rename = "timeout")]
    #[serde(default = "default_timeout")]
    pub timeout: i64,
    /// HTTP monitors: URL requested.
    #[serde(rename = "url")]
    #[serde(default = "default_url")]
    pub url: String,
    /// HTTP monitors: method the URL is requested with. Null for other types.
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::enums::MonitorMethod>,
    /// HTTP monitors: headers sent, as an object of header names and values.
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// HTTP monitors: request body sent.
    #[serde(rename = "body")]
    #[serde(default = "default_body")]
    pub body: String,
    /// DNS monitors: domain queried.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// DNS monitors: record type queried. Null for other types.
    #[serde(rename = "record")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<crate::enums::MonitorRecord>,
    /// DNS monitors: resolver queried. Empty for the server's DNS resolver.
    #[serde(rename = "resolver")]
    #[serde(default = "default_resolver")]
    pub resolver: String,
    /// TCP and SMTP monitors: hostname or IP address connected to.
    #[serde(rename = "host")]
    #[serde(default = "default_host")]
    pub host: String,
    /// TCP and SMTP monitors: port connected to.
    #[serde(rename = "port")]
    #[serde(default = "default_port")]
    pub port: i64,
    /// SMTP monitors: how the connection is secured. `auto` picks implicit TLS on
    /// port 465, STARTTLS on 587 and none otherwise. Null for other types.
    #[serde(rename = "tlsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_mode: Option<crate::enums::MonitorTlsMode>,
    /// Probe monitors: what the last check found. `ok` passes; anything else
    /// opened the monitor's alert. Null before the first check.
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::enums::MonitorResult>,
    /// Probe monitors: milliseconds the last check took.
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// Probe monitors: last status code an HTTP target answered with, reply code
    /// an SMTP server sent, or response code a DNS resolver returned.
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// Probe monitors: expiry date of the TLS certificate the last check saw, in
    /// ISO 8601 format. Null without TLS.
    #[serde(rename = "certificateExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expires_at: Option<String>,
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

fn default_name() -> String {
    "".to_string()
}

fn default_project_id() -> String {
    "".to_string()
}

fn default_rules() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_type() -> crate::enums::MonitorType {
    serde_json::from_value(serde_json::json!("usage")).unwrap()
}

fn default_resource_type() -> crate::enums::MonitorResourceType {
    serde_json::from_value(serde_json::json!("project")).unwrap()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_window() -> i64 {
    3600
}

fn default_cooldown() -> i64 {
    3600
}

fn default_enabled() -> bool {
    true
}

fn default_notify() -> Vec<String> {
    serde_json::from_value(serde_json::json!(["owner"])).unwrap()
}

fn default_escalation_id() -> String {
    "".to_string()
}

fn default_severity() -> crate::enums::MonitorSeverity {
    serde_json::from_value(serde_json::json!("warning")).unwrap()
}

fn default_status() -> crate::enums::MonitorStatus {
    serde_json::from_value(serde_json::json!("ok")).unwrap()
}

fn default_alert_id() -> String {
    "".to_string()
}

fn default_interval() -> i64 {
    0
}

fn default_timeout() -> i64 {
    0
}

fn default_url() -> String {
    "".to_string()
}

fn default_body() -> String {
    "".to_string()
}

fn default_domain() -> String {
    "".to_string()
}

fn default_resolver() -> String {
    "".to_string()
}

fn default_host() -> String {
    "".to_string()
}

fn default_port() -> i64 {
    0
}

impl Monitor {
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

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get rules
    pub fn rules(&self) -> &Vec<String> {
        &self.rules
    }

    /// Get r#type
    pub fn r#type(&self) -> &crate::enums::MonitorType {
        &self.r#type
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &crate::enums::MonitorResourceType {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get window
    pub fn window(&self) -> &i64 {
        &self.window
    }

    /// Get cooldown
    pub fn cooldown(&self) -> &i64 {
        &self.cooldown
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get notify
    pub fn notify(&self) -> &Vec<String> {
        &self.notify
    }

    /// Get escalation_id
    pub fn escalation_id(&self) -> &String {
        &self.escalation_id
    }

    /// Get severity
    pub fn severity(&self) -> &crate::enums::MonitorSeverity {
        &self.severity
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::MonitorStatus {
        &self.status
    }

    /// Set value
    pub fn set_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Get value
    pub fn value(&self) -> Option<&f64> {
        self.value.as_ref()
    }

    /// Set evaluated_at
    pub fn set_evaluated_at(mut self, evaluated_at: String) -> Self {
        self.evaluated_at = Some(evaluated_at);
        self
    }

    /// Get evaluated_at
    pub fn evaluated_at(&self) -> Option<&String> {
        self.evaluated_at.as_ref()
    }

    /// Get alert_id
    pub fn alert_id(&self) -> &String {
        &self.alert_id
    }

    /// Set triggered_at
    pub fn set_triggered_at(mut self, triggered_at: String) -> Self {
        self.triggered_at = Some(triggered_at);
        self
    }

    /// Get triggered_at
    pub fn triggered_at(&self) -> Option<&String> {
        self.triggered_at.as_ref()
    }

    /// Get interval
    pub fn interval(&self) -> &i64 {
        &self.interval
    }

    /// Get timeout
    pub fn timeout(&self) -> &i64 {
        &self.timeout
    }

    /// Get url
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Set method
    pub fn set_method(mut self, method: crate::enums::MonitorMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Get method
    pub fn method(&self) -> Option<&crate::enums::MonitorMethod> {
        self.method.as_ref()
    }

    /// Set headers
    pub fn set_headers(mut self, headers: serde_json::Value) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Get headers
    pub fn headers(&self) -> Option<&serde_json::Value> {
        self.headers.as_ref()
    }

    /// Get body
    pub fn body(&self) -> &String {
        &self.body
    }

    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Set record
    pub fn set_record(mut self, record: crate::enums::MonitorRecord) -> Self {
        self.record = Some(record);
        self
    }

    /// Get record
    pub fn record(&self) -> Option<&crate::enums::MonitorRecord> {
        self.record.as_ref()
    }

    /// Get resolver
    pub fn resolver(&self) -> &String {
        &self.resolver
    }

    /// Get host
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Get port
    pub fn port(&self) -> &i64 {
        &self.port
    }

    /// Set tls_mode
    pub fn set_tls_mode(mut self, tls_mode: crate::enums::MonitorTlsMode) -> Self {
        self.tls_mode = Some(tls_mode);
        self
    }

    /// Get tls_mode
    pub fn tls_mode(&self) -> Option<&crate::enums::MonitorTlsMode> {
        self.tls_mode.as_ref()
    }

    /// Set result
    pub fn set_result(mut self, result: crate::enums::MonitorResult) -> Self {
        self.result = Some(result);
        self
    }

    /// Get result
    pub fn result(&self) -> Option<&crate::enums::MonitorResult> {
        self.result.as_ref()
    }

    /// Set latency
    pub fn set_latency(mut self, latency: i64) -> Self {
        self.latency = Some(latency);
        self
    }

    /// Get latency
    pub fn latency(&self) -> Option<&i64> {
        self.latency.as_ref()
    }

    /// Set code
    pub fn set_code(mut self, code: i64) -> Self {
        self.code = Some(code);
        self
    }

    /// Get code
    pub fn code(&self) -> Option<&i64> {
        self.code.as_ref()
    }

    /// Set certificate_expires_at
    pub fn set_certificate_expires_at(mut self, certificate_expires_at: String) -> Self {
        self.certificate_expires_at = Some(certificate_expires_at);
        self
    }

    /// Get certificate_expires_at
    pub fn certificate_expires_at(&self) -> Option<&String> {
        self.certificate_expires_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let _model = <Monitor as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.project_id();
        let _ = _model.rules();
        let _ = _model.r#type();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.window();
        let _ = _model.cooldown();
        let _ = _model.enabled();
        let _ = _model.notify();
        let _ = _model.escalation_id();
        let _ = _model.severity();
        let _ = _model.status();
        let _ = _model.alert_id();
        let _ = _model.interval();
        let _ = _model.timeout();
        let _ = _model.url();
        let _ = _model.body();
        let _ = _model.domain();
        let _ = _model.resolver();
        let _ = _model.host();
        let _ = _model.port();
    }

    #[test]
    fn test_monitor_serialization() {
        let model = <Monitor as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Monitor, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
