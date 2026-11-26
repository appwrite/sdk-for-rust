//! Alert model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Alert {
    /// Alert ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Date the alert started firing, in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Alert update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// ID of the monitor that raised the alert.
    #[serde(rename = "monitorId")]
    #[serde(default = "default_monitor_id")]
    pub monitor_id: String,
    /// ID of the project the alert is about. Empty for an alert of the whole
    /// organization.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Name of the monitor when the alert fired.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Whether the alert is still firing or has resolved.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: crate::enums::AlertStatus,
    /// Date the alert resolved, in ISO 8601 format. Empty while it fires.
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Date a member acknowledged the alert, in ISO 8601 format. Empty while
    /// nobody has.
    #[serde(rename = "acknowledgedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    /// ID of the user who acknowledged the alert. Empty while nobody has, or when
    /// it was acknowledged with an API key.
    #[serde(rename = "acknowledgedBy")]
    #[serde(default = "default_acknowledged_by")]
    pub acknowledged_by: String,
    /// Name of the user who acknowledged the alert, when they did.
    #[serde(rename = "acknowledgedByName")]
    #[serde(default = "default_acknowledged_by_name")]
    pub acknowledged_by_name: String,
    /// ID of the escalation notifying the team of this alert. Empty when the
    /// monitor notified its `notify` roles instead.
    #[serde(rename = "escalationId")]
    #[serde(default = "default_escalation_id")]
    pub escalation_id: String,
    /// Index of the escalation's next step to notify, counting from 0. Equal to
    /// the number of steps once the last round has gone out.
    #[serde(rename = "escalationStep")]
    #[serde(default = "default_escalation_step")]
    pub escalation_step: i64,
    /// How many times the escalation's steps have been repeated for this alert.
    #[serde(rename = "escalationRound")]
    #[serde(default = "default_escalation_round")]
    pub escalation_round: i64,
    /// When the next step is notified, in ISO 8601 format. Null when nothing is
    /// due: the alert was acknowledged or resolved, or every step has gone out.
    #[serde(rename = "escalatesAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalates_at: Option<String>,
    /// The monitor's rules when the alert fired, as query strings.
    #[serde(rename = "rules")]
    #[serde(default = "default_rules")]
    pub rules: Vec<String>,
    /// The monitor's severity when the alert fired: `info`, `warning`, `error` or
    /// `critical`.
    #[serde(rename = "severity")]
    #[serde(default = "default_severity")]
    pub severity: crate::enums::MonitorSeverity,
    /// Metric the alert fired on. Null for a probe monitor's alert.
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<crate::enums::AlertMetric>,
    /// How the metric was compared with the threshold. Null for a probe monitor's
    /// alert.
    #[serde(rename = "comparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<crate::enums::AlertComparator>,
    /// Threshold the metric crossed, in the metric's unit.
    #[serde(rename = "threshold")]
    #[serde(default = "default_threshold")]
    pub threshold: f64,
    /// Latest reading while the alert fired: the metric's value for a usage
    /// monitor, and for a probe monitor its `latency`.
    #[serde(rename = "value")]
    #[serde(default = "default_value")]
    pub value: f64,
    /// Reading furthest past the threshold while the alert fired.
    #[serde(rename = "peak")]
    #[serde(default = "default_peak")]
    pub peak: f64,
    /// Type of the monitor that raised the alert.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: crate::enums::MonitorType,
    /// Type of the resource the alert fired on: the project, or one database,
    /// bucket, function or site.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: crate::enums::MonitorResourceType,
    /// ID of the resource the alert fired on. The project ID for a project-wide
    /// monitor.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// How many times the alert was announced as firing: once when it fired, and
    /// again after each cooldown while it kept firing.
    #[serde(rename = "notifications")]
    #[serde(default = "default_notifications")]
    pub notifications: i64,
    /// Probe monitors: response time in milliseconds of the latest check recorded
    /// on the alert.
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// What the failing check of a probe monitor found when the alert opened, such
    /// as `connect_refused`. Null for a usage monitor's alert.
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::enums::MonitorResult>,
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

fn default_monitor_id() -> String {
    "".to_string()
}

fn default_project_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_status() -> crate::enums::AlertStatus {
    serde_json::from_value(serde_json::json!("firing")).unwrap()
}

fn default_acknowledged_by() -> String {
    "".to_string()
}

fn default_acknowledged_by_name() -> String {
    "".to_string()
}

fn default_escalation_id() -> String {
    "".to_string()
}

fn default_escalation_step() -> i64 {
    0
}

fn default_escalation_round() -> i64 {
    0
}

fn default_rules() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_severity() -> crate::enums::MonitorSeverity {
    serde_json::from_value(serde_json::json!("warning")).unwrap()
}

fn default_threshold() -> f64 {
    0.0
}

fn default_value() -> f64 {
    0.0
}

fn default_peak() -> f64 {
    0.0
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

fn default_notifications() -> i64 {
    0
}

impl Alert {
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

    /// Get monitor_id
    pub fn monitor_id(&self) -> &String {
        &self.monitor_id
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::AlertStatus {
        &self.status
    }

    /// Set resolved_at
    pub fn set_resolved_at(mut self, resolved_at: String) -> Self {
        self.resolved_at = Some(resolved_at);
        self
    }

    /// Get resolved_at
    pub fn resolved_at(&self) -> Option<&String> {
        self.resolved_at.as_ref()
    }

    /// Set acknowledged_at
    pub fn set_acknowledged_at(mut self, acknowledged_at: String) -> Self {
        self.acknowledged_at = Some(acknowledged_at);
        self
    }

    /// Get acknowledged_at
    pub fn acknowledged_at(&self) -> Option<&String> {
        self.acknowledged_at.as_ref()
    }

    /// Get acknowledged_by
    pub fn acknowledged_by(&self) -> &String {
        &self.acknowledged_by
    }

    /// Get acknowledged_by_name
    pub fn acknowledged_by_name(&self) -> &String {
        &self.acknowledged_by_name
    }

    /// Get escalation_id
    pub fn escalation_id(&self) -> &String {
        &self.escalation_id
    }

    /// Get escalation_step
    pub fn escalation_step(&self) -> &i64 {
        &self.escalation_step
    }

    /// Get escalation_round
    pub fn escalation_round(&self) -> &i64 {
        &self.escalation_round
    }

    /// Set escalates_at
    pub fn set_escalates_at(mut self, escalates_at: String) -> Self {
        self.escalates_at = Some(escalates_at);
        self
    }

    /// Get escalates_at
    pub fn escalates_at(&self) -> Option<&String> {
        self.escalates_at.as_ref()
    }

    /// Get rules
    pub fn rules(&self) -> &Vec<String> {
        &self.rules
    }

    /// Get severity
    pub fn severity(&self) -> &crate::enums::MonitorSeverity {
        &self.severity
    }

    /// Set metric
    pub fn set_metric(mut self, metric: crate::enums::AlertMetric) -> Self {
        self.metric = Some(metric);
        self
    }

    /// Get metric
    pub fn metric(&self) -> Option<&crate::enums::AlertMetric> {
        self.metric.as_ref()
    }

    /// Set comparator
    pub fn set_comparator(mut self, comparator: crate::enums::AlertComparator) -> Self {
        self.comparator = Some(comparator);
        self
    }

    /// Get comparator
    pub fn comparator(&self) -> Option<&crate::enums::AlertComparator> {
        self.comparator.as_ref()
    }

    /// Get threshold
    pub fn threshold(&self) -> &f64 {
        &self.threshold
    }

    /// Get value
    pub fn value(&self) -> &f64 {
        &self.value
    }

    /// Get peak
    pub fn peak(&self) -> &f64 {
        &self.peak
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

    /// Get notifications
    pub fn notifications(&self) -> &i64 {
        &self.notifications
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

    /// Set result
    pub fn set_result(mut self, result: crate::enums::MonitorResult) -> Self {
        self.result = Some(result);
        self
    }

    /// Get result
    pub fn result(&self) -> Option<&crate::enums::MonitorResult> {
        self.result.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let _model = <Alert as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.monitor_id();
        let _ = _model.project_id();
        let _ = _model.name();
        let _ = _model.status();
        let _ = _model.acknowledged_by();
        let _ = _model.acknowledged_by_name();
        let _ = _model.escalation_id();
        let _ = _model.escalation_step();
        let _ = _model.escalation_round();
        let _ = _model.rules();
        let _ = _model.severity();
        let _ = _model.threshold();
        let _ = _model.value();
        let _ = _model.peak();
        let _ = _model.r#type();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.notifications();
    }

    #[test]
    fn test_alert_serialization() {
        let model = <Alert as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Alert, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
