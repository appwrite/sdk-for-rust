use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DatabaseStatus {
    #[serde(rename = "provisioning")]
    #[default]
    Provisioning,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "restoring")]
    Restoring,
    #[serde(rename = "scaling")]
    Scaling,
    #[serde(rename = "upgrading")]
    Upgrading,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "pausing")]
    Pausing,
    #[serde(rename = "resuming")]
    Resuming,
    #[serde(rename = "failing-over")]
    FailingOver,
}

impl DatabaseStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DatabaseStatus::Provisioning => "provisioning",
            DatabaseStatus::Ready => "ready",
            DatabaseStatus::Inactive => "inactive",
            DatabaseStatus::Paused => "paused",
            DatabaseStatus::Failed => "failed",
            DatabaseStatus::Deleting => "deleting",
            DatabaseStatus::Deleted => "deleted",
            DatabaseStatus::Restoring => "restoring",
            DatabaseStatus::Scaling => "scaling",
            DatabaseStatus::Upgrading => "upgrading",
            DatabaseStatus::Migrating => "migrating",
            DatabaseStatus::Pausing => "pausing",
            DatabaseStatus::Resuming => "resuming",
            DatabaseStatus::FailingOver => "failing-over",
        }
    }
}

impl std::fmt::Display for DatabaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
