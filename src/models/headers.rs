//! Headers model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Headers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Headers {
    /// Header name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Header value.
    #[serde(rename = "value")]
    #[serde(default = "default_value")]
    pub value: String,
}

fn default_name() -> String {
    "".to_string()
}

fn default_value() -> String {
    "".to_string()
}

impl Headers {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get value
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers_creation() {
        let _model = <Headers as Default>::default();
        let _ = _model.name();
        let _ = _model.value();
    }

    #[test]
    fn test_headers_serialization() {
        let model = <Headers as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Headers, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
