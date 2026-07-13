use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DatabaseStatus {
    #[serde(rename = "provisioning")]
    #[default]
    Provisioning,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "failed")]
    Failed,
}

impl DatabaseStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DatabaseStatus::Provisioning => "provisioning",
            DatabaseStatus::Ready => "ready",
            DatabaseStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for DatabaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
