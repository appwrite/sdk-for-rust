use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HealthAntivirusStatus {
    #[serde(rename = "disabled")]
    #[default]
    Disabled,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
}

impl HealthAntivirusStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            HealthAntivirusStatus::Disabled => "disabled",
            HealthAntivirusStatus::Offline => "offline",
            HealthAntivirusStatus::Online => "online",
        }
    }
}

impl std::fmt::Display for HealthAntivirusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
