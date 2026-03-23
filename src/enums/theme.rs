use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Theme {
    #[serde(rename = "light")]
    #[default]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

impl Theme {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
