//! Execution model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Execution {
    /// Execution ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Execution creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Execution update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Execution roles.
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,
    /// Function ID.
    #[serde(rename = "functionId")]
    pub function_id: String,
    /// Function's deployment ID used to create the execution.
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// The trigger that caused the function to execute. Possible values can be:
    /// `http`, `schedule`, or `event`.
    #[serde(rename = "trigger")]
    pub trigger: crate::enums::ExecutionTrigger,
    /// The status of the function execution. Possible values can be: `waiting`,
    /// `processing`, `completed`, `failed`, or `scheduled`.
    #[serde(rename = "status")]
    pub status: crate::enums::ExecutionStatus,
    /// HTTP request method type.
    #[serde(rename = "requestMethod")]
    pub request_method: String,
    /// HTTP request path and query.
    #[serde(rename = "requestPath")]
    pub request_path: String,
    /// HTTP request headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<crate::models::Headers>,
    /// HTTP response status code.
    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i64,
    /// HTTP response body. This will return empty unless execution is created as
    /// synchronous.
    #[serde(rename = "responseBody")]
    pub response_body: String,
    /// HTTP response headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "responseHeaders")]
    pub response_headers: Vec<crate::models::Headers>,
    /// Function logs. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "logs")]
    pub logs: String,
    /// Function errors. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "errors")]
    pub errors: String,
    /// Resource(function/site) execution duration in seconds.
    #[serde(rename = "duration")]
    pub duration: f64,
    /// The scheduled time for execution. If left empty, execution will be queued
    /// immediately.
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
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

    /// Get function_id
    pub fn function_id(&self) -> &String {
        &self.function_id
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
        let _ = _model.function_id();
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
