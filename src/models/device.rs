//! Device model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Device {
    /// Device ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Device creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Device update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Device expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
    /// ID of the session that is currently active on the device. Empty when no
    /// session is active, for example right after the active session was deleted.
    #[serde(rename = "activeSessionId")]
    #[serde(default = "default_active_session_id")]
    pub active_session_id: String,
    /// List of sessions the device holds. Switch between them to change the active
    /// account.
    #[serde(rename = "sessions")]
    #[serde(default = "default_sessions")]
    pub sessions: Vec<crate::models::Session>,
    /// Secret used to authenticate the device. Carried by the device cookie; only
    /// included in the response if the request was made with an API key.
    #[serde(rename = "secret")]
    #[serde(default = "default_secret")]
    pub secret: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

fn default_active_session_id() -> String {
    "".to_string()
}

fn default_sessions() -> Vec<crate::models::Session> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_secret() -> String {
    "".to_string()
}

impl Device {
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

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get active_session_id
    pub fn active_session_id(&self) -> &String {
        &self.active_session_id
    }

    /// Get sessions
    pub fn sessions(&self) -> &Vec<crate::models::Session> {
        &self.sessions
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let _model = <Device as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.expire();
        let _ = _model.active_session_id();
        let _ = _model.sessions();
        let _ = _model.secret();
    }

    #[test]
    fn test_device_serialization() {
        let model = <Device as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Device, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
