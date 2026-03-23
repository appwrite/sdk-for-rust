//! WebhookList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Webhooks List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct WebhookList {
    /// Total number of webhooks that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of webhooks.
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<crate::models::Webhook>,
}

impl WebhookList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get webhooks
    pub fn webhooks(&self) -> &Vec<crate::models::Webhook> {
        &self.webhooks
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_list_creation() {
        let _model = <WebhookList as Default>::default();
        let _ = _model.total();
        let _ = _model.webhooks();
    }

    #[test]
    fn test_webhook_list_serialization() {
        let model = <WebhookList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<WebhookList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
