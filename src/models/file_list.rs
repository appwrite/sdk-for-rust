//! FileList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Files List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct FileList {
    /// Total number of files that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of files.
    #[serde(rename = "files")]
    pub files: Vec<crate::models::File>,
}

impl FileList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get files
    pub fn files(&self) -> &Vec<crate::models::File> {
        &self.files
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_list_creation() {
        let _model = <FileList as Default>::default();
        let _ = _model.total();
        let _ = _model.files();
    }

    #[test]
    fn test_file_list_serialization() {
        let model = <FileList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<FileList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
