//! Continent model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Continent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Continent {
    /// Continent name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Continent two letter code.
    #[serde(rename = "code")]
    #[serde(default = "default_code")]
    pub code: String,
}

fn default_name() -> String {
    "".to_string()
}

fn default_code() -> String {
    "".to_string()
}

impl Continent {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get code
    pub fn code(&self) -> &String {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continent_creation() {
        let _model = <Continent as Default>::default();
        let _ = _model.name();
        let _ = _model.code();
    }

    #[test]
    fn test_continent_serialization() {
        let model = <Continent as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Continent, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
