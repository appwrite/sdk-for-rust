use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IdTokenProvider {
    #[serde(rename = "apple")]
    #[default]
    Apple,
    #[serde(rename = "google")]
    Google,
}

impl IdTokenProvider {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            IdTokenProvider::Apple => "apple",
            IdTokenProvider::Google => "google",
        }
    }
}

impl std::fmt::Display for IdTokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
