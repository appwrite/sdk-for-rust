//! BackupArchiveList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Backup archive list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupArchiveList {
    /// Total number of archives that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of archives.
    #[serde(rename = "archives")]
    pub archives: Vec<crate::models::BackupArchive>,
}

impl BackupArchiveList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get archives
    pub fn archives(&self) -> &Vec<crate::models::BackupArchive> {
        &self.archives
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_archive_list_creation() {
        let _model = <BackupArchiveList as Default>::default();
        let _ = _model.total();
        let _ = _model.archives();
    }

    #[test]
    fn test_backup_archive_list_serialization() {
        let model = <BackupArchiveList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BackupArchiveList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
