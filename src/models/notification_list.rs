//! NotificationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Notifications List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct NotificationList {
    /// Total number of notifications that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of notifications.
    #[serde(rename = "notifications")]
    pub notifications: Vec<crate::models::Notification>,
}

impl NotificationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get notifications
    pub fn notifications(&self) -> &Vec<crate::models::Notification> {
        &self.notifications
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_list_creation() {
        let _model = <NotificationList as Default>::default();
        let _ = _model.total();
        let _ = _model.notifications();
    }

    #[test]
    fn test_notification_list_serialization() {
        let model = <NotificationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<NotificationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
