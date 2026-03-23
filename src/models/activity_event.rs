//! ActivityEvent model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ActivityEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ActivityEvent {
    /// Event ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// User type.
    #[serde(rename = "userType")]
    pub user_type: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// User Email.
    #[serde(rename = "userEmail")]
    pub user_email: String,
    /// User Name.
    #[serde(rename = "userName")]
    pub user_name: String,
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
    /// Operating system code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode")]
    pub os_code: String,
    /// Operating system name.
    #[serde(rename = "osName")]
    pub os_name: String,
    /// Operating system version.
    #[serde(rename = "osVersion")]
    pub os_version: String,
    /// Client type.
    #[serde(rename = "clientType")]
    pub client_type: String,
    /// Client code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode")]
    pub client_code: String,
    /// Client name.
    #[serde(rename = "clientName")]
    pub client_name: String,
    /// Client version.
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    /// Client engine name.
    #[serde(rename = "clientEngine")]
    pub client_engine: String,
    /// Client engine name.
    #[serde(rename = "clientEngineVersion")]
    pub client_engine_version: String,
    /// Device name.
    #[serde(rename = "deviceName")]
    pub device_name: String,
    /// Device brand name.
    #[serde(rename = "deviceBrand")]
    pub device_brand: String,
    /// Device model name.
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName")]
    pub country_name: String,
}

impl ActivityEvent {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get user_type
    pub fn user_type(&self) -> &String {
        &self.user_type
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get user_email
    pub fn user_email(&self) -> &String {
        &self.user_email
    }

    /// Get user_name
    pub fn user_name(&self) -> &String {
        &self.user_name
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

    /// Get os_code
    pub fn os_code(&self) -> &String {
        &self.os_code
    }

    /// Get os_name
    pub fn os_name(&self) -> &String {
        &self.os_name
    }

    /// Get os_version
    pub fn os_version(&self) -> &String {
        &self.os_version
    }

    /// Get client_type
    pub fn client_type(&self) -> &String {
        &self.client_type
    }

    /// Get client_code
    pub fn client_code(&self) -> &String {
        &self.client_code
    }

    /// Get client_name
    pub fn client_name(&self) -> &String {
        &self.client_name
    }

    /// Get client_version
    pub fn client_version(&self) -> &String {
        &self.client_version
    }

    /// Get client_engine
    pub fn client_engine(&self) -> &String {
        &self.client_engine
    }

    /// Get client_engine_version
    pub fn client_engine_version(&self) -> &String {
        &self.client_engine_version
    }

    /// Get device_name
    pub fn device_name(&self) -> &String {
        &self.device_name
    }

    /// Get device_brand
    pub fn device_brand(&self) -> &String {
        &self.device_brand
    }

    /// Get device_model
    pub fn device_model(&self) -> &String {
        &self.device_model
    }

    /// Get country_code
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Get country_name
    pub fn country_name(&self) -> &String {
        &self.country_name
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_event_creation() {
        let _model = <ActivityEvent as Default>::default();
        let _ = _model.id();
        let _ = _model.user_type();
        let _ = _model.user_id();
        let _ = _model.user_email();
        let _ = _model.user_name();
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
        let _ = _model.os_code();
        let _ = _model.os_name();
        let _ = _model.os_version();
        let _ = _model.client_type();
        let _ = _model.client_code();
        let _ = _model.client_name();
        let _ = _model.client_version();
        let _ = _model.client_engine();
        let _ = _model.client_engine_version();
        let _ = _model.device_name();
        let _ = _model.device_brand();
        let _ = _model.device_model();
        let _ = _model.country_code();
        let _ = _model.country_name();
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
