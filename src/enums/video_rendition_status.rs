use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VideoRenditionStatus {
    #[serde(rename = "pending")]
    #[default]
    Pending,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "ended")]
    Ended,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "aborted")]
    Aborted,
}

impl VideoRenditionStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VideoRenditionStatus::Pending => "pending",
            VideoRenditionStatus::Started => "started",
            VideoRenditionStatus::Ended => "ended",
            VideoRenditionStatus::Ready => "ready",
            VideoRenditionStatus::Error => "error",
            VideoRenditionStatus::Aborted => "aborted",
        }
    }
}

impl std::fmt::Display for VideoRenditionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
