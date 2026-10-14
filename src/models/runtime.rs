//! Runtime model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Runtime {
    /// Runtime ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Parent runtime key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
    /// Runtime Name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Runtime version.
    #[serde(rename = "version")]
    #[serde(default = "default_version")]
    pub version: String,
    /// Base Docker image used to build the runtime.
    #[serde(rename = "base")]
    #[serde(default = "default_base")]
    pub base: String,
    /// Image name of Docker Hub.
    #[serde(rename = "image")]
    #[serde(default = "default_image")]
    pub image: String,
    /// Name of the logo image.
    #[serde(rename = "logo")]
    #[serde(default = "default_logo")]
    pub logo: String,
    /// List of supported architectures.
    #[serde(rename = "supports")]
    #[serde(default = "default_supports")]
    pub supports: Vec<String>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_key() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_version() -> String {
    "".to_string()
}

fn default_base() -> String {
    "".to_string()
}

fn default_image() -> String {
    "".to_string()
}

fn default_logo() -> String {
    "".to_string()
}

fn default_supports() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl Runtime {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get base
    pub fn base(&self) -> &String {
        &self.base
    }

    /// Get image
    pub fn image(&self) -> &String {
        &self.image
    }

    /// Get logo
    pub fn logo(&self) -> &String {
        &self.logo
    }

    /// Get supports
    pub fn supports(&self) -> &Vec<String> {
        &self.supports
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let _model = <Runtime as Default>::default();
        let _ = _model.id();
        let _ = _model.key();
        let _ = _model.name();
        let _ = _model.version();
        let _ = _model.base();
        let _ = _model.image();
        let _ = _model.logo();
        let _ = _model.supports();
    }

    #[test]
    fn test_runtime_serialization() {
        let model = <Runtime as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Runtime, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
