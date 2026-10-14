//! ProxyInvalidation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProxyInvalidation {
    /// Domain name.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// Invalidation type. Possible values are "tag", "path", or "all".
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Invalidated reference. Depending on type this is a cache tag name, a URL
    /// path, or empty when type is all.
    #[serde(rename = "reference")]
    #[serde(default = "default_reference")]
    pub reference: String,
    /// Invalidation status.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_domain() -> String {
    "".to_string()
}

fn default_type() -> String {
    "".to_string()
}

fn default_reference() -> String {
    "".to_string()
}

fn default_status() -> String {
    "".to_string()
}

impl ProxyInvalidation {
    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get reference
    pub fn reference(&self) -> &String {
        &self.reference
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_invalidation_creation() {
        let _model = <ProxyInvalidation as Default>::default();
        let _ = _model.domain();
        let _ = _model.r#type();
        let _ = _model.reference();
        let _ = _model.status();
    }

    #[test]
    fn test_proxy_invalidation_serialization() {
        let model = <ProxyInvalidation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProxyInvalidation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
