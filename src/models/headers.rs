//! Headers model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Headers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Headers {
    /// Header name.
    #[serde(rename = "name")]
    pub name: String,
    /// Header value.
    #[serde(rename = "value")]
    pub value: String,
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
