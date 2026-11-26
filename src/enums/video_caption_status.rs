use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VideoCaptionStatus {
    #[serde(rename = "pending")]
    #[default]
    Pending,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "aborted")]
    Aborted,
}

impl VideoCaptionStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VideoCaptionStatus::Pending => "pending",
            VideoCaptionStatus::Started => "started",
            VideoCaptionStatus::Ready => "ready",
            VideoCaptionStatus::Error => "error",
            VideoCaptionStatus::Aborted => "aborted",
        }
    }
}

impl std::fmt::Display for VideoCaptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
