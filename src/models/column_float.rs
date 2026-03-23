//! ColumnFloat model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ColumnFloat
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ColumnFloat {
    /// Column Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Column type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Column status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: crate::enums::ColumnStatus,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an column.
    #[serde(rename = "error")]
    pub error: String,
    /// Is column required?
    #[serde(rename = "required")]
    pub required: bool,
    /// Is column an array?
    #[serde(rename = "array")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    /// Column creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Column update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Minimum value to enforce for new documents.
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum value to enforce for new documents.
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Default value for column when not provided. Cannot be set when column is
    /// required.
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f64>,
}

impl ColumnFloat {
    /// Get key
    pub fn key(&self) -> &String {
        &self.key
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

    /// Set min
    pub fn set_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Get min
    pub fn min(&self) -> Option<&f64> {
        self.min.as_ref()
    }

    /// Set max
    pub fn set_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Get max
    pub fn max(&self) -> Option<&f64> {
        self.max.as_ref()
    }

    /// Set default
    pub fn set_default(mut self, default: f64) -> Self {
        self.default = Some(default);
        self
    }

    /// Get default
    pub fn default(&self) -> Option<&f64> {
        self.default.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_float_creation() {
        let _model = <ColumnFloat as Default>::default();
        let _ = _model.key();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.required();
        let _ = _model.created_at();
        let _ = _model.updated_at();
    }

    #[test]
    fn test_column_float_serialization() {
        let model = <ColumnFloat as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ColumnFloat, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
