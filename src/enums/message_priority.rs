use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MessagePriority {
    #[serde(rename = "normal")]
    #[default]
    Normal,
    #[serde(rename = "high")]
    High,
}

impl MessagePriority {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MessagePriority::Normal => "normal",
            MessagePriority::High => "high",
        }
    }
}

impl std::fmt::Display for MessagePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
