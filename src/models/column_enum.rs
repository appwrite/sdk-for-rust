//! ColumnEnum model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ColumnEnum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ColumnEnum {
    /// Column Key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
    /// Optional description for the column.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Column type.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Column status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: crate::enums::ColumnStatus,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an column.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
    /// Is column required?
    #[serde(rename = "required")]
    #[serde(default = "default_required")]
    pub required: bool,
    /// Is column an array?
    #[serde(rename = "array")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    /// Column creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Column update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Array of elements in enumerated type.
    #[serde(rename = "elements")]
    #[serde(default = "default_elements")]
    pub elements: Vec<String>,
    /// String format.
    #[serde(rename = "format")]
    #[serde(default = "default_format")]
    pub format: String,
    /// Default value for column when not provided. Cannot be set when column is
    /// required.
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

fn default_key() -> String {
    "".to_string()
}

fn default_description() -> String {
    "".to_string()
}

fn default_type() -> String {
    "".to_string()
}

fn default_error() -> String {
    "".to_string()
}

fn default_required() -> bool {
    false
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_elements() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_format() -> String {
    "enum".to_string()
}

impl ColumnEnum {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::ColumnStatus {
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

    /// Get elements
    pub fn elements(&self) -> &Vec<String> {
        &self.elements
    }

    /// Get format
    pub fn format(&self) -> &String {
        &self.format
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_enum_creation() {
        let _model = <ColumnEnum as Default>::default();
        let _ = _model.key();
        let _ = _model.description();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.required();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.elements();
        let _ = _model.format();
    }

    #[test]
    fn test_column_enum_serialization() {
        let model = <ColumnEnum as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ColumnEnum, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
