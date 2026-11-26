use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AlertStatus {
    #[serde(rename = "firing")]
    #[default]
    Firing,
    #[serde(rename = "resolved")]
    Resolved,
}

impl AlertStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AlertStatus::Firing => "firing",
            AlertStatus::Resolved => "resolved",
        }
    }
}

impl std::fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
