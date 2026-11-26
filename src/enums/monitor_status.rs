use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorStatus {
    #[serde(rename = "ok")]
    #[default]
    Ok,
    #[serde(rename = "firing")]
    Firing,
}

impl MonitorStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorStatus::Ok => "ok",
            MonitorStatus::Firing => "firing",
        }
    }
}

impl std::fmt::Display for MonitorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
