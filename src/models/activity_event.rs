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
    /// Continent code.
    #[serde(rename = "continentCode")]
    pub continent_code: String,
    /// City name.
    #[serde(rename = "city")]
    pub city: String,
    /// Region/state chain.
    #[serde(rename = "subdivisions")]
    pub subdivisions: String,
    /// Internet service provider.
    #[serde(rename = "isp")]
    pub isp: String,
    /// Autonomous System Number (ASN).
    #[serde(rename = "autonomousSystemNumber")]
    pub autonomous_system_number: String,
    /// Organization that owns the ASN.
    #[serde(rename = "autonomousSystemOrganization")]
    pub autonomous_system_organization: String,
    /// Connection type (e.g. cable, cellular, corporate).
    #[serde(rename = "connectionType")]
    pub connection_type: String,
    /// User type (e.g. residential, business, hosting).
    #[serde(rename = "connectionUsageType")]
    pub connection_usage_type: String,
    /// Registered organization of the IP.
    #[serde(rename = "connectionOrganization")]
    pub connection_organization: String,
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
    /// Name of the SDK that triggered the event.
    #[serde(rename = "sdk")]
    pub sdk: String,
    /// Version of the SDK that triggered the event.
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
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

    /// Get continent_code
    pub fn continent_code(&self) -> &String {
        &self.continent_code
    }

    /// Get city
    pub fn city(&self) -> &String {
        &self.city
    }

    /// Get subdivisions
    pub fn subdivisions(&self) -> &String {
        &self.subdivisions
    }

    /// Get isp
    pub fn isp(&self) -> &String {
        &self.isp
    }

    /// Get autonomous_system_number
    pub fn autonomous_system_number(&self) -> &String {
        &self.autonomous_system_number
    }

    /// Get autonomous_system_organization
    pub fn autonomous_system_organization(&self) -> &String {
        &self.autonomous_system_organization
    }

    /// Get connection_type
    pub fn connection_type(&self) -> &String {
        &self.connection_type
    }

    /// Get connection_usage_type
    pub fn connection_usage_type(&self) -> &String {
        &self.connection_usage_type
    }

    /// Get connection_organization
    pub fn connection_organization(&self) -> &String {
        &self.connection_organization
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

    /// Get sdk
    pub fn sdk(&self) -> &String {
        &self.sdk
    }

    /// Get sdk_version
    pub fn sdk_version(&self) -> &String {
        &self.sdk_version
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
        let _ = _model.continent_code();
        let _ = _model.city();
        let _ = _model.subdivisions();
        let _ = _model.isp();
        let _ = _model.autonomous_system_number();
        let _ = _model.autonomous_system_organization();
        let _ = _model.connection_type();
        let _ = _model.connection_usage_type();
        let _ = _model.connection_organization();
        let _ = _model.time();
        let _ = _model.project_id();
        let _ = _model.team_id();
        let _ = _model.hostname();
        let _ = _model.sdk();
        let _ = _model.sdk_version();
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
