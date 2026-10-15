//! Execution model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Execution {
    /// Execution ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Execution creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Execution update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Execution roles.
    #[serde(rename = "$permissions")]
    #[serde(default = "default_permissions")]
    pub permissions: Vec<String>,
    /// Function or site ID.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Execution resource type.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: crate::enums::ExecutionResourceType,
    /// Deployment ID used to create the execution.
    #[serde(rename = "deploymentId")]
    #[serde(default = "default_deployment_id")]
    pub deployment_id: String,
    /// The trigger that caused the resource to execute. Possible values can be:
    /// `http`, `schedule`, `event`, or `email`.
    #[serde(rename = "trigger")]
    pub trigger: crate::enums::ExecutionTrigger,
    /// The status of the resource execution. Possible values can be: `waiting`,
    /// `processing`, `completed`, `failed`, or `scheduled`.
    #[serde(rename = "status")]
    pub status: crate::enums::ExecutionStatus,
    /// HTTP request method type.
    #[serde(rename = "requestMethod")]
    #[serde(default = "default_request_method")]
    pub request_method: String,
    /// HTTP request path and query.
    #[serde(rename = "requestPath")]
    #[serde(default = "default_request_path")]
    pub request_path: String,
    /// HTTP request headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "requestHeaders")]
    #[serde(default = "default_request_headers")]
    pub request_headers: Vec<crate::models::Headers>,
    /// HTTP response status code.
    #[serde(rename = "responseStatusCode")]
    #[serde(default = "default_response_status_code")]
    pub response_status_code: i64,
    /// HTTP response body. This will return empty unless execution is created as
    /// synchronous.
    #[serde(rename = "responseBody")]
    #[serde(default = "default_response_body")]
    pub response_body: String,
    /// HTTP response headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "responseHeaders")]
    #[serde(default = "default_response_headers")]
    pub response_headers: Vec<crate::models::Headers>,
    /// Resource logs. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "logs")]
    #[serde(default = "default_logs")]
    pub logs: String,
    /// Resource errors. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "errors")]
    #[serde(default = "default_errors")]
    pub errors: String,
    /// Total time the resource(function/site) took to respond, in seconds.
    #[serde(rename = "duration")]
    #[serde(default = "default_duration")]
    pub duration: f64,
    /// The scheduled time for execution. If left empty, execution will be queued
    /// immediately.
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
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

fn default_permissions() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_resource_type() -> crate::enums::ExecutionResourceType {
    serde_json::from_value(serde_json::json!("functions")).unwrap()
}

fn default_deployment_id() -> String {
    "".to_string()
}

fn default_request_method() -> String {
    "".to_string()
}

fn default_request_path() -> String {
    "".to_string()
}

fn default_request_headers() -> Vec<crate::models::Headers> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_response_status_code() -> i64 {
    0
}

fn default_response_body() -> String {
    "".to_string()
}

fn default_response_headers() -> Vec<crate::models::Headers> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_logs() -> String {
    "".to_string()
}

fn default_errors() -> String {
    "".to_string()
}

fn default_duration() -> f64 {
    0.0
}

impl Execution {
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

    /// Get permissions
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &crate::enums::ExecutionResourceType {
        &self.resource_type
    }

    /// Get deployment_id
    pub fn deployment_id(&self) -> &String {
        &self.deployment_id
    }

    /// Get trigger
    pub fn trigger(&self) -> &crate::enums::ExecutionTrigger {
        &self.trigger
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::ExecutionStatus {
        &self.status
    }

    /// Get request_method
    pub fn request_method(&self) -> &String {
        &self.request_method
    }

    /// Get request_path
    pub fn request_path(&self) -> &String {
        &self.request_path
    }

    /// Get request_headers
    pub fn request_headers(&self) -> &Vec<crate::models::Headers> {
        &self.request_headers
    }

    /// Get response_status_code
    pub fn response_status_code(&self) -> &i64 {
        &self.response_status_code
    }

    /// Get response_body
    pub fn response_body(&self) -> &String {
        &self.response_body
    }

    /// Get response_headers
    pub fn response_headers(&self) -> &Vec<crate::models::Headers> {
        &self.response_headers
    }

    /// Get logs
    pub fn logs(&self) -> &String {
        &self.logs
    }

    /// Get errors
    pub fn errors(&self) -> &String {
        &self.errors
    }

    /// Get duration
    pub fn duration(&self) -> &f64 {
        &self.duration
    }

    /// Set scheduled_at
    pub fn set_scheduled_at(mut self, scheduled_at: String) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }

    /// Get scheduled_at
    pub fn scheduled_at(&self) -> Option<&String> {
        self.scheduled_at.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_creation() {
        let _model = <Execution as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.resource_id();
        let _ = _model.resource_type();
        let _ = _model.deployment_id();
        let _ = _model.trigger();
        let _ = _model.status();
        let _ = _model.request_method();
        let _ = _model.request_path();
        let _ = _model.request_headers();
        let _ = _model.response_status_code();
        let _ = _model.response_body();
        let _ = _model.response_headers();
        let _ = _model.logs();
        let _ = _model.errors();
        let _ = _model.duration();
    }

    #[test]
    fn test_execution_serialization() {
        let model = <Execution as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Execution, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
