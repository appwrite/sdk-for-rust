use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Adapter {
    #[serde(rename = "static")]
    #[default]
    Static,
    #[serde(rename = "ssr")]
    Ssr,
}

impl Adapter {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Adapter::Static => "static",
            Adapter::Ssr => "ssr",
        }
    }
}

impl std::fmt::Display for Adapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
