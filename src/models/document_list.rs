//! DocumentList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Documents List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DocumentList {
    /// Total number of documents that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of documents.
    #[serde(rename = "documents")]
    pub documents: Vec<crate::models::Document>,
}

impl DocumentList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get documents
    pub fn documents(&self) -> &Vec<crate::models::Document> {
        &self.documents
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_list_creation() {
        let _model = <DocumentList as Default>::default();
        let _ = _model.total();
        let _ = _model.documents();
    }

    #[test]
    fn test_document_list_serialization() {
        let model = <DocumentList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DocumentList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
