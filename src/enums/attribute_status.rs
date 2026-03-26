use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AttributeStatus {
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

impl AttributeStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AttributeStatus::Available => "available",
            AttributeStatus::Processing => "processing",
            AttributeStatus::Deleting => "deleting",
            AttributeStatus::Stuck => "stuck",
            AttributeStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for AttributeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
