use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColumnStatus {
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

impl ColumnStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ColumnStatus::Available => "available",
            ColumnStatus::Processing => "processing",
            ColumnStatus::Deleting => "deleting",
            ColumnStatus::Stuck => "stuck",
            ColumnStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for ColumnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
