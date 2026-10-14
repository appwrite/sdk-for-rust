//! PlatformAndroid model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Platform Android
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PlatformAndroid {
    /// Platform ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Platform creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Platform update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Platform name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Platform type. Possible values are: windows, apple, android, linux, web.
    #[serde(rename = "type")]
    pub r#type: crate::enums::PlatformType,
    /// Android application ID.
    #[serde(rename = "applicationId")]
    #[serde(default = "default_application_id")]
    pub application_id: String,
    /// SHA-256 fingerprints of the certificates the app is signed with. Passkeys
    /// created in the app are accepted when its signature matches one.
    #[serde(rename = "fingerprints")]
    #[serde(default = "default_fingerprints")]
    pub fingerprints: Vec<String>,
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

fn default_name() -> String {
    "".to_string()
}

fn default_application_id() -> String {
    "".to_string()
}

fn default_fingerprints() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl PlatformAndroid {
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

    /// Get r#type
    pub fn r#type(&self) -> &crate::enums::PlatformType {
        &self.r#type
    }

    /// Get application_id
    pub fn application_id(&self) -> &String {
        &self.application_id
    }

    /// Get fingerprints
    pub fn fingerprints(&self) -> &Vec<String> {
        &self.fingerprints
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_android_creation() {
        let _model = <PlatformAndroid as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.r#type();
        let _ = _model.application_id();
        let _ = _model.fingerprints();
    }

    #[test]
    fn test_platform_android_serialization() {
        let model = <PlatformAndroid as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PlatformAndroid, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
