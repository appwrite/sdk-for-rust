use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DeploymentDownloadType {
    #[serde(rename = "source")]
    #[default]
    Source,
    #[serde(rename = "output")]
    Output,
}

impl DeploymentDownloadType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DeploymentDownloadType::Source => "source",
            DeploymentDownloadType::Output => "output",
        }
    }
}

impl std::fmt::Display for DeploymentDownloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
