use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorTlsMode {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "starttls")]
    Starttls,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "auto")]
    Auto,
}

impl MonitorTlsMode {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorTlsMode::None => "none",
            MonitorTlsMode::Starttls => "starttls",
            MonitorTlsMode::Implicit => "implicit",
            MonitorTlsMode::Auto => "auto",
        }
    }
}

impl std::fmt::Display for MonitorTlsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
