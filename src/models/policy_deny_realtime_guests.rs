//! PolicyDenyRealtimeGuests model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Deny Realtime Guests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyDenyRealtimeGuests {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Whether guests are stopped from subscribing to realtime channels.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_id() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    false
}

impl PolicyDenyRealtimeGuests {
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
    fn test_policy_deny_realtime_guests_creation() {
        let _model = <PolicyDenyRealtimeGuests as Default>::default();
        let _ = _model.id();
        let _ = _model.enabled();
    }

    #[test]
    fn test_policy_deny_realtime_guests_serialization() {
        let model = <PolicyDenyRealtimeGuests as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyDenyRealtimeGuests, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
