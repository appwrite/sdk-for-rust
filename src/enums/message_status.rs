use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MessageStatus {
    #[serde(rename = "draft")]
    #[default]
    Draft,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "failed")]
    Failed,
}

impl MessageStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MessageStatus::Draft => "draft",
            MessageStatus::Processing => "processing",
            MessageStatus::Scheduled => "scheduled",
            MessageStatus::Sent => "sent",
            MessageStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
