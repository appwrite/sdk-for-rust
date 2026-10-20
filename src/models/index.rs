//! Index model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Index {
    /// Index ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Index creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Index update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Index key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
    /// Optional description for the index.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Index type.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Index status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: crate::enums::IndexStatus,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an index.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
    /// Index attributes.
    #[serde(rename = "attributes")]
    #[serde(default = "default_attributes")]
    pub attributes: Vec<String>,
    /// Index attributes length.
    #[serde(rename = "lengths")]
    #[serde(default = "default_lengths")]
    pub lengths: Vec<i64>,
    /// Index orders.
    #[serde(rename = "orders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<String>>,
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

fn default_attributes() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_lengths() -> Vec<i64> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl Index {
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
    pub fn status(&self) -> &crate::enums::IndexStatus {
        &self.status
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }

    /// Get attributes
    pub fn attributes(&self) -> &Vec<String> {
        &self.attributes
    }

    /// Get lengths
    pub fn lengths(&self) -> &Vec<i64> {
        &self.lengths
    }

    /// Set orders
    pub fn set_orders(mut self, orders: Vec<String>) -> Self {
        self.orders = Some(orders);
        self
    }

    /// Get orders
    pub fn orders(&self) -> Option<&Vec<String>> {
        self.orders.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_creation() {
        let _model = <Index as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.key();
        let _ = _model.description();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.attributes();
        let _ = _model.lengths();
    }

    #[test]
    fn test_index_serialization() {
        let model = <Index as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Index, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
