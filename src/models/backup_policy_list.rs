//! BackupPolicyList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Backup policy list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BackupPolicyList {
    /// Total number of policies that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of policies.
    #[serde(rename = "policies")]
    pub policies: Vec<crate::models::BackupPolicy>,
}

impl BackupPolicyList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get policies
    pub fn policies(&self) -> &Vec<crate::models::BackupPolicy> {
        &self.policies
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_policy_list_creation() {
        let _model = <BackupPolicyList as Default>::default();
        let _ = _model.total();
        let _ = _model.policies();
    }

    #[test]
    fn test_backup_policy_list_serialization() {
        let model = <BackupPolicyList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BackupPolicyList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
