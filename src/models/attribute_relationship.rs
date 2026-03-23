//! AttributeRelationship model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AttributeRelationship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AttributeRelationship {
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
    /// The ID of the related collection.
    #[serde(rename = "relatedCollection")]
    pub related_collection: String,
    /// The type of the relationship.
    #[serde(rename = "relationType")]
    pub relation_type: String,
    /// Is the relationship two-way?
    #[serde(rename = "twoWay")]
    pub two_way: bool,
    /// The key of the two-way relationship.
    #[serde(rename = "twoWayKey")]
    pub two_way_key: String,
    /// How deleting the parent document will propagate to child documents.
    #[serde(rename = "onDelete")]
    pub on_delete: String,
    /// Whether this is the parent or child side of the relationship
    #[serde(rename = "side")]
    pub side: String,
}

impl AttributeRelationship {
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

    /// Get related_collection
    pub fn related_collection(&self) -> &String {
        &self.related_collection
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
    fn test_attribute_relationship_creation() {
        let _model = <AttributeRelationship as Default>::default();
        let _ = _model.key();
        let _ = _model.r#type();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.required();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.related_collection();
        let _ = _model.relation_type();
        let _ = _model.two_way();
        let _ = _model.two_way_key();
        let _ = _model.on_delete();
        let _ = _model.side();
    }

    #[test]
    fn test_attribute_relationship_serialization() {
        let model = <AttributeRelationship as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AttributeRelationship, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
