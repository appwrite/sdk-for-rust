//! Framework model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Framework
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Framework {
    /// Framework key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
    /// Framework Name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Default runtime version.
    #[serde(rename = "buildRuntime")]
    #[serde(default = "default_build_runtime")]
    pub build_runtime: String,
    /// List of supported runtime versions.
    #[serde(rename = "runtimes")]
    pub runtimes: Vec<String>,
    /// List of supported adapters.
    #[serde(rename = "adapters")]
    pub adapters: Vec<crate::models::FrameworkAdapter>,
}

fn default_key() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_build_runtime() -> String {
    "".to_string()
}

impl Framework {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get build_runtime
    pub fn build_runtime(&self) -> &String {
        &self.build_runtime
    }

    /// Get runtimes
    pub fn runtimes(&self) -> &Vec<String> {
        &self.runtimes
    }

    /// Get adapters
    pub fn adapters(&self) -> &Vec<crate::models::FrameworkAdapter> {
        &self.adapters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_creation() {
        let _model = <Framework as Default>::default();
        let _ = _model.key();
        let _ = _model.name();
        let _ = _model.build_runtime();
        let _ = _model.runtimes();
        let _ = _model.adapters();
    }

    #[test]
    fn test_framework_serialization() {
        let model = <Framework as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Framework, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
