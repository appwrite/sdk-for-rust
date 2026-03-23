//! FrameworkAdapter model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Framework Adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct FrameworkAdapter {
    /// Adapter key.
    #[serde(rename = "key")]
    pub key: String,
    /// Default command to download dependencies.
    #[serde(rename = "installCommand")]
    pub install_command: String,
    /// Default command to build site into output directory.
    #[serde(rename = "buildCommand")]
    pub build_command: String,
    /// Default output directory of build.
    #[serde(rename = "outputDirectory")]
    pub output_directory: String,
    /// Name of fallback file to use instead of 404 page. If null, Appwrite 404
    /// page will be displayed.
    #[serde(rename = "fallbackFile")]
    pub fallback_file: String,
}

impl FrameworkAdapter {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get install_command
    pub fn install_command(&self) -> &String {
        &self.install_command
    }

    /// Get build_command
    pub fn build_command(&self) -> &String {
        &self.build_command
    }

    /// Get output_directory
    pub fn output_directory(&self) -> &String {
        &self.output_directory
    }

    /// Get fallback_file
    pub fn fallback_file(&self) -> &String {
        &self.fallback_file
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_adapter_creation() {
        let _model = <FrameworkAdapter as Default>::default();
        let _ = _model.key();
        let _ = _model.install_command();
        let _ = _model.build_command();
        let _ = _model.output_directory();
        let _ = _model.fallback_file();
    }

    #[test]
    fn test_framework_adapter_serialization() {
        let model = <FrameworkAdapter as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<FrameworkAdapter, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
