//! PlatformWeb model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Platform Web
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PlatformWeb {
    /// Platform ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Platform creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Platform update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Platform name.
    #[serde(rename = "name")]
    pub name: String,
    /// Platform type. Possible values are: windows, apple, android, linux, web.
    #[serde(rename = "type")]
    pub r#type: crate::enums::PlatformType,
    /// Web app hostname. Empty string for other platforms.
    #[serde(rename = "hostname")]
    pub hostname: String,
}

impl PlatformWeb {
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

    /// Get hostname
    pub fn hostname(&self) -> &String {
        &self.hostname
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_web_creation() {
        let _model = <PlatformWeb as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.r#type();
        let _ = _model.hostname();
    }

    #[test]
    fn test_platform_web_serialization() {
        let model = <PlatformWeb as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PlatformWeb, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
