use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VideoTimelineStatus {
    #[serde(rename = "pending")]
    #[default]
    Pending,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "error")]
    Error,
}

impl VideoTimelineStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VideoTimelineStatus::Pending => "pending",
            VideoTimelineStatus::Started => "started",
            VideoTimelineStatus::Ready => "ready",
            VideoTimelineStatus::Skipped => "skipped",
            VideoTimelineStatus::Error => "error",
        }
    }
}

impl std::fmt::Display for VideoTimelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
