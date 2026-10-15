use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ExecutionTrigger {
    #[serde(rename = "http")]
    #[default]
    Http,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "email")]
    Email,
}

impl ExecutionTrigger {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionTrigger::Http => "http",
            ExecutionTrigger::Schedule => "schedule",
            ExecutionTrigger::Event => "event",
            ExecutionTrigger::Email => "email",
        }
    }
}

impl std::fmt::Display for ExecutionTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
