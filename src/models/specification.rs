//! Specification model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Specification {
    /// Memory size in MB.
    #[serde(rename = "memory")]
    #[serde(default = "default_memory")]
    pub memory: i64,
    /// Number of CPUs.
    #[serde(rename = "cpus")]
    #[serde(default = "default_cpus")]
    pub cpus: f64,
    /// Is size enabled.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Size slug.
    #[serde(rename = "slug")]
    #[serde(default = "default_slug")]
    pub slug: String,
}

fn default_memory() -> i64 {
    0
}

fn default_cpus() -> f64 {
    0.0
}

fn default_enabled() -> bool {
    false
}

fn default_slug() -> String {
    "".to_string()
}

impl Specification {
    /// Get memory
    pub fn memory(&self) -> &i64 {
        &self.memory
    }

    /// Get cpus
    pub fn cpus(&self) -> &f64 {
        &self.cpus
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get slug
    pub fn slug(&self) -> &String {
        &self.slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specification_creation() {
        let _model = <Specification as Default>::default();
        let _ = _model.memory();
        let _ = _model.cpus();
        let _ = _model.enabled();
        let _ = _model.slug();
    }

    #[test]
    fn test_specification_serialization() {
        let model = <Specification as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Specification, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
