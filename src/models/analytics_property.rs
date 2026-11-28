//! AnalyticsProperty model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AnalyticsProperty
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsProperty {
    /// Analytics property ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Property creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Property update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Human-readable name of the tracked website or application.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Primary domain being tracked (e.g. example.com). May be empty for mobile
    /// and agent properties.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// Whether tracking is currently active.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// List of origins allowed to send tracking events. Use ["*"] to allow all.
    /// Managed by Appwrite for a property linked to a site, following the site's
    /// production domains.
    #[serde(rename = "allowedOrigins")]
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,
    /// Kind of product this property tracks. Decides which dashboard panels apply.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: crate::enums::AnalyticsPropertyType,
    /// Whether events sent from a signed-in client session record that user's ID.
    #[serde(rename = "identifyUsers")]
    #[serde(default = "default_identify_users")]
    pub identify_users: bool,
    /// Most recent event date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt")]
    #[serde(default = "default_accessed_at")]
    pub accessed_at: String,
    /// First event date in ISO 8601 format. Empty until the property receives its
    /// first event.
    #[serde(rename = "firstAccessedAt")]
    #[serde(default = "default_first_accessed_at")]
    pub first_accessed_at: String,
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

fn default_domain() -> String {
    "".to_string()
}

fn default_enabled() -> bool {
    true
}

fn default_allowed_origins() -> Vec<String> {
    serde_json::from_value(serde_json::json!(["*"])).unwrap()
}

fn default_type() -> crate::enums::AnalyticsPropertyType {
    serde_json::from_value(serde_json::json!("web")).unwrap()
}

fn default_identify_users() -> bool {
    false
}

fn default_accessed_at() -> String {
    "".to_string()
}

fn default_first_accessed_at() -> String {
    "".to_string()
}

impl AnalyticsProperty {
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

    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get allowed_origins
    pub fn allowed_origins(&self) -> &Vec<String> {
        &self.allowed_origins
    }

    /// Get r#type
    pub fn r#type(&self) -> &crate::enums::AnalyticsPropertyType {
        &self.r#type
    }

    /// Get identify_users
    pub fn identify_users(&self) -> &bool {
        &self.identify_users
    }

    /// Get accessed_at
    pub fn accessed_at(&self) -> &String {
        &self.accessed_at
    }

    /// Get first_accessed_at
    pub fn first_accessed_at(&self) -> &String {
        &self.first_accessed_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_property_creation() {
        let _model = <AnalyticsProperty as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.domain();
        let _ = _model.enabled();
        let _ = _model.allowed_origins();
        let _ = _model.r#type();
        let _ = _model.identify_users();
        let _ = _model.accessed_at();
        let _ = _model.first_accessed_at();
    }

    #[test]
    fn test_analytics_property_serialization() {
        let model = <AnalyticsProperty as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsProperty, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
