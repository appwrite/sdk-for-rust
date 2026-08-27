//! DedicatedDatabaseBackupList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// BackupList
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseBackupList {
    /// Total number of backups.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of backups.
    #[serde(rename = "backups")]
    pub backups: Vec<crate::models::DedicatedDatabaseBackup>,
}

impl DedicatedDatabaseBackupList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get backups
    pub fn backups(&self) -> &Vec<crate::models::DedicatedDatabaseBackup> {
        &self.backups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_backup_list_creation() {
        let _model = <DedicatedDatabaseBackupList as Default>::default();
        let _ = _model.total();
        let _ = _model.backups();
    }

    #[test]
    fn test_dedicated_database_backup_list_serialization() {
        let model = <DedicatedDatabaseBackupList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseBackupList, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
