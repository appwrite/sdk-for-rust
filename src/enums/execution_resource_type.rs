use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ExecutionResourceType {
    #[serde(rename = "functions")]
    #[default]
    Functions,
    #[serde(rename = "sites")]
    Sites,
}

impl ExecutionResourceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionResourceType::Functions => "functions",
            ExecutionResourceType::Sites => "sites",
        }
    }
}

impl std::fmt::Display for ExecutionResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
