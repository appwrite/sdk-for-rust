//! ColumnRelationship model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// ColumnRelationship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ColumnRelationship {
    /// Column Key.
    #[serde(rename = "key")]
    #[serde(default = "default_key")]
    pub key: String,
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
    /// The ID of the related table.
    #[serde(rename = "relatedTable")]
    pub related_table: String,
    /// The type of the relationship.
    #[serde(rename = "relationType")]
    #[serde(default = "default_relation_type")]
    pub relation_type: String,
    /// Is the relationship two-way?
    #[serde(rename = "twoWay")]
    #[serde(default = "default_two_way")]
    pub two_way: bool,
    /// The key of the two-way relationship.
    #[serde(rename = "twoWayKey")]
    #[serde(default = "default_two_way_key")]
    pub two_way_key: String,
    /// How deleting the parent document will propagate to child documents.
    #[serde(rename = "onDelete")]
    #[serde(default = "default_on_delete")]
    pub on_delete: String,
    /// Whether this is the parent or child side of the relationship
    #[serde(rename = "side")]
    #[serde(default = "default_side")]
    pub side: String,
}

fn default_key() -> String {
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

fn default_relation_type() -> String {
    "".to_string()
}

fn default_two_way() -> bool {
    false
}

fn default_two_way_key() -> String {
    "".to_string()
}

fn default_on_delete() -> String {
    "restrict".to_string()
}

fn default_side() -> String {
    "".to_string()
}

impl ColumnRelationship {
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

    /// Get related_table
    pub fn related_table(&self) -> &String {
        &self.related_table
    }

    /// Get relation_type
    pub fn relation_type(&self) -> &String {
        &self.relation_type
    }

    /// Get two_way
    pub fn two_way(&self) -> &bool {
        &self.two_way
    }

    /// Get two_way_key
    pub fn two_way_key(&self) -> &String {
        &self.two_way_key
    }

    /// Get on_delete
    pub fn on_delete(&self) -> &String {
        &self.on_delete
    }

    /// Get side
    pub fn side(&self) -> &String {
        &self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_relationship_creation() {
        let _model = <ColumnRelationship as Default>::default();
        let _ = _model.key();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.required();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.related_table();
        let _ = _model.relation_type();
        let _ = _model.two_way();
        let _ = _model.two_way_key();
        let _ = _model.on_delete();
        let _ = _model.side();
    }

    #[test]
    fn test_column_relationship_serialization() {
        let model = <ColumnRelationship as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ColumnRelationship, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
