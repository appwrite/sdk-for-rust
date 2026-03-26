use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DeploymentStatus {
    #[serde(rename = "waiting")]
    #[default]
    Waiting,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
}

impl DeploymentStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DeploymentStatus::Waiting => "waiting",
            DeploymentStatus::Processing => "processing",
            DeploymentStatus::Building => "building",
            DeploymentStatus::Ready => "ready",
            DeploymentStatus::Canceled => "canceled",
            DeploymentStatus::Failed => "failed",
        }
    }
}

impl std::fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
