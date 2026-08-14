use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum InvalidationType {
    #[serde(rename = "tag")]
    #[default]
    Tag,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "all")]
    All,
}

impl InvalidationType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            InvalidationType::Tag => "tag",
            InvalidationType::Path => "path",
            InvalidationType::All => "all",
        }
    }
}

impl std::fmt::Display for InvalidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
