//! ActivityEvent model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ActivityEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ActivityEvent {
    /// Event ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Actor type.
    #[serde(rename = "actorType")]
    pub actor_type: String,
    /// Actor ID.
    #[serde(rename = "actorId")]
    pub actor_id: String,
    /// Actor Email.
    #[serde(rename = "actorEmail")]
    pub actor_email: String,
    /// Actor Name.
    #[serde(rename = "actorName")]
    pub actor_name: String,
    /// Resource parent.
    #[serde(rename = "resourceParent")]
    pub resource_parent: String,
    /// Resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Resource ID.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Resource.
    #[serde(rename = "resource")]
    pub resource: String,
    /// Event name.
    #[serde(rename = "event")]
    pub event: String,
    /// User agent.
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    /// IP address.
    #[serde(rename = "ip")]
    pub ip: String,
    /// API mode when event triggered.
    #[serde(rename = "mode")]
    pub mode: String,
    /// Location.
    #[serde(rename = "country")]
    pub country: String,
    /// Log creation date in ISO 8601 format.
    #[serde(rename = "time")]
    pub time: String,
    /// Project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Hostname.
    #[serde(rename = "hostname")]
    pub hostname: String,
}

impl ActivityEvent {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get actor_type
    pub fn actor_type(&self) -> &String {
        &self.actor_type
    }

    /// Get actor_id
    pub fn actor_id(&self) -> &String {
        &self.actor_id
    }

    /// Get actor_email
    pub fn actor_email(&self) -> &String {
        &self.actor_email
    }

    /// Get actor_name
    pub fn actor_name(&self) -> &String {
        &self.actor_name
    }

    /// Get resource_parent
    pub fn resource_parent(&self) -> &String {
        &self.resource_parent
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get resource
    pub fn resource(&self) -> &String {
        &self.resource
    }

    /// Get event
    pub fn event(&self) -> &String {
        &self.event
    }

    /// Get user_agent
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }

    /// Get ip
    pub fn ip(&self) -> &String {
        &self.ip
    }

    /// Get mode
    pub fn mode(&self) -> &String {
        &self.mode
    }

    /// Get country
    pub fn country(&self) -> &String {
        &self.country
    }

    /// Get time
    pub fn time(&self) -> &String {
        &self.time
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get hostname
    pub fn hostname(&self) -> &String {
        &self.hostname
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_event_creation() {
        let _model = <ActivityEvent as Default>::default();
        let _ = _model.id();
        let _ = _model.actor_type();
        let _ = _model.actor_id();
        let _ = _model.actor_email();
        let _ = _model.actor_name();
        let _ = _model.resource_parent();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.resource();
        let _ = _model.event();
        let _ = _model.user_agent();
        let _ = _model.ip();
        let _ = _model.mode();
        let _ = _model.country();
        let _ = _model.time();
        let _ = _model.project_id();
        let _ = _model.team_id();
        let _ = _model.hostname();
    }

    #[test]
    fn test_activity_event_serialization() {
        let model = <ActivityEvent as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ActivityEvent, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
