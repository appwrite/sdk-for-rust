//! UsageDataPoint model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageDataPoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageDataPoint {
    /// Bucket start timestamp (ISO 8601). When `interval` is omitted this is the
    /// request end time, marking the aggregate as-of moment.
    #[serde(rename = "time")]
    pub time: String,
    /// Aggregated value for the bucket.
    #[serde(rename = "value")]
    pub value: i64,
    /// API endpoint path when broken down by `path`.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// HTTP method when broken down by `method`.
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// HTTP status code when broken down by `status`.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// API service segment when broken down by `service`.
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Country code when broken down by `country`.
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Appwrite region when broken down by `region`.
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Caller origin hostname when broken down by `hostname`.
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Operating system name when broken down by `osName`.
    #[serde(rename = "osName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Client type when broken down by `clientType`.
    #[serde(rename = "clientType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_type: Option<String>,
    /// Client name when broken down by `clientName`.
    #[serde(rename = "clientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Device classification when broken down by `deviceName`.
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Owning team ID when broken down by `teamId`.
    #[serde(rename = "teamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// External resource ID when broken down by `resourceId`.
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Resource type when broken down by `resource` (gauges only).
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

impl UsageDataPoint {
    /// Get time
    pub fn time(&self) -> &String {
        &self.time
    }

    /// Get value
    pub fn value(&self) -> &i64 {
        &self.value
    }

    /// Set path
    pub fn set_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    /// Get path
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Set method
    pub fn set_method(mut self, method: String) -> Self {
        self.method = Some(method);
        self
    }

    /// Get method
    pub fn method(&self) -> Option<&String> {
        self.method.as_ref()
    }

    /// Set status
    pub fn set_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// Get status
    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Set service
    pub fn set_service(mut self, service: String) -> Self {
        self.service = Some(service);
        self
    }

    /// Get service
    pub fn service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Set country
    pub fn set_country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    /// Get country
    pub fn country(&self) -> Option<&String> {
        self.country.as_ref()
    }

    /// Set region
    pub fn set_region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }

    /// Get region
    pub fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    /// Set hostname
    pub fn set_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    /// Get hostname
    pub fn hostname(&self) -> Option<&String> {
        self.hostname.as_ref()
    }

    /// Set os_name
    pub fn set_os_name(mut self, os_name: String) -> Self {
        self.os_name = Some(os_name);
        self
    }

    /// Get os_name
    pub fn os_name(&self) -> Option<&String> {
        self.os_name.as_ref()
    }

    /// Set client_type
    pub fn set_client_type(mut self, client_type: String) -> Self {
        self.client_type = Some(client_type);
        self
    }

    /// Get client_type
    pub fn client_type(&self) -> Option<&String> {
        self.client_type.as_ref()
    }

    /// Set client_name
    pub fn set_client_name(mut self, client_name: String) -> Self {
        self.client_name = Some(client_name);
        self
    }

    /// Get client_name
    pub fn client_name(&self) -> Option<&String> {
        self.client_name.as_ref()
    }

    /// Set device_name
    pub fn set_device_name(mut self, device_name: String) -> Self {
        self.device_name = Some(device_name);
        self
    }

    /// Get device_name
    pub fn device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Set team_id
    pub fn set_team_id(mut self, team_id: String) -> Self {
        self.team_id = Some(team_id);
        self
    }

    /// Get team_id
    pub fn team_id(&self) -> Option<&String> {
        self.team_id.as_ref()
    }

    /// Set resource_id
    pub fn set_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Get resource_id
    pub fn resource_id(&self) -> Option<&String> {
        self.resource_id.as_ref()
    }

    /// Set resource
    pub fn set_resource(mut self, resource: String) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Get resource
    pub fn resource(&self) -> Option<&String> {
        self.resource.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_data_point_creation() {
        let _model = <UsageDataPoint as Default>::default();
        let _ = _model.time();
        let _ = _model.value();
    }

    #[test]
    fn test_usage_data_point_serialization() {
        let model = <UsageDataPoint as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageDataPoint, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
