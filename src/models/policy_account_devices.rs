//! PolicyAccountDevices model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Account Devices
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyAccountDevices {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Whether account devices are enabled. When disabled, new account devices can
    /// no longer be created.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    true
}

impl PolicyAccountDevices {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_account_devices_creation() {
        let _model = <PolicyAccountDevices as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_account_devices_serialization() {
        let model = <PolicyAccountDevices as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyAccountDevices, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
