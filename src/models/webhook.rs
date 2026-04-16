//! Webhook model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Webhook {
    /// Webhook ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Webhook creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Webhook update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Webhook name.
    #[serde(rename = "name")]
    pub name: String,
    /// Webhook URL endpoint.
    #[serde(rename = "url")]
    pub url: String,
    /// Webhook trigger events.
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// Indicates if SSL / TLS certificate verification is enabled.
    #[serde(rename = "tls")]
    pub tls: bool,
    /// HTTP basic authentication username.
    #[serde(rename = "authUsername")]
    pub auth_username: String,
    /// HTTP basic authentication password.
    #[serde(rename = "authPassword")]
    pub auth_password: String,
    /// Signature key which can be used to validate incoming webhook payloads. Only
    /// returned on creation and secret rotation.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Indicates if this webhook is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Webhook error logs from the most recent failure.
    #[serde(rename = "logs")]
    pub logs: String,
    /// Number of consecutive failed webhook attempts.
    #[serde(rename = "attempts")]
    pub attempts: i64,
}

impl Webhook {
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

    /// Get url
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Get events
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }

    /// Get tls
    pub fn tls(&self) -> &bool {
        &self.tls
    }

    /// Get auth_username
    pub fn auth_username(&self) -> &String {
        &self.auth_username
    }

    /// Get auth_password
    pub fn auth_password(&self) -> &String {
        &self.auth_password
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get logs
    pub fn logs(&self) -> &String {
        &self.logs
    }

    /// Get attempts
    pub fn attempts(&self) -> &i64 {
        &self.attempts
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_creation() {
        let _model = <Webhook as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.url();
        let _ = _model.events();
        let _ = _model.tls();
        let _ = _model.auth_username();
        let _ = _model.auth_password();
        let _ = _model.secret();
        let _ = _model.enabled();
        let _ = _model.logs();
        let _ = _model.attempts();
    }

    #[test]
    fn test_webhook_serialization() {
        let model = <Webhook as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Webhook, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
