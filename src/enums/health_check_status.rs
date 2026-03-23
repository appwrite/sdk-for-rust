use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HealthCheckStatus {
    #[serde(rename = "pass")]
    #[default]
    Pass,
    #[serde(rename = "fail")]
    Fail,
}

impl HealthCheckStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            HealthCheckStatus::Pass => "pass",
            HealthCheckStatus::Fail => "fail",
        }
    }
}

impl std::fmt::Display for HealthCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
