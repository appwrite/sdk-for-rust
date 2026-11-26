use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorType {
    #[serde(rename = "usage")]
    #[default]
    Usage,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "smtp")]
    Smtp,
    #[serde(rename = "budget")]
    Budget,
}

impl MonitorType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorType::Usage => "usage",
            MonitorType::Http => "http",
            MonitorType::Dns => "dns",
            MonitorType::Tcp => "tcp",
            MonitorType::Smtp => "smtp",
            MonitorType::Budget => "budget",
        }
    }
}

impl std::fmt::Display for MonitorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
