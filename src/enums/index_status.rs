use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IndexStatus {
    #[serde(rename = "available")]
    #[default]
    Available,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "stuck")]
    Stuck,
    #[serde(rename = "failed")]
    Failed,
}

impl IndexStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            IndexStatus::Available => "available",
            IndexStatus::Processing => "processing",
            IndexStatus::Deleting => "deleting",
            IndexStatus::Stuck => "stuck",
            IndexStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for IndexStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
