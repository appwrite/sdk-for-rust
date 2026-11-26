use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorSeverity {
    #[serde(rename = "info")]
    #[default]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
}

impl MonitorSeverity {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorSeverity::Info => "info",
            MonitorSeverity::Warning => "warning",
            MonitorSeverity::Error => "error",
            MonitorSeverity::Critical => "critical",
        }
    }
}

impl std::fmt::Display for MonitorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
