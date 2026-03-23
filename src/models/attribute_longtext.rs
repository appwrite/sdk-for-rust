//! AttributeLongtext model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AttributeLongtext
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AttributeLongtext {
    /// Attribute Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Attribute type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Attribute status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: crate::enums::AttributeStatus,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an attribute.
    #[serde(rename = "error")]
    pub error: String,
    /// Is attribute required?
    #[serde(rename = "required")]
    pub required: bool,
    /// Is attribute an array?
    #[serde(rename = "array")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    /// Attribute creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Attribute update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Default value for attribute when not provided. Cannot be set when attribute
    /// is required.
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Defines whether this attribute is encrypted or not.
    #[serde(rename = "encrypt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt: Option<bool>,
}

impl AttributeLongtext {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::AttributeStatus {
        &self.status
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }

    /// Get required
    pub fn required(&self) -> &bool {
        &self.required
    }

    /// Set array
    pub fn set_array(mut self, array: bool) -> Self {
        self.array = Some(array);
        self
    }

    /// Get array
    pub fn array(&self) -> Option<&bool> {
        self.array.as_ref()
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Set default
    pub fn set_default(mut self, default: String) -> Self {
        self.default = Some(default);
        self
    }

    /// Get default
    pub fn default(&self) -> Option<&String> {
        self.default.as_ref()
    }

    /// Set encrypt
    pub fn set_encrypt(mut self, encrypt: bool) -> Self {
        self.encrypt = Some(encrypt);
        self
    }

    /// Get encrypt
    pub fn encrypt(&self) -> Option<&bool> {
        self.encrypt.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_longtext_creation() {
        let _model = <AttributeLongtext as Default>::default();
        let _ = _model.key();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.required();
        let _ = _model.created_at();
        let _ = _model.updated_at();
    }

    #[test]
    fn test_attribute_longtext_serialization() {
        let model = <AttributeLongtext as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AttributeLongtext, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
