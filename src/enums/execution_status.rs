use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ExecutionStatus {
    #[serde(rename = "waiting")]
    #[default]
    Waiting,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "scheduled")]
    Scheduled,
}

impl ExecutionStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionStatus::Waiting => "waiting",
            ExecutionStatus::Processing => "processing",
            ExecutionStatus::Completed => "completed",
            ExecutionStatus::Failed => "failed",
            ExecutionStatus::Scheduled => "scheduled",
        }
    }
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
