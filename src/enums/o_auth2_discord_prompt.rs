use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2DiscordPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "consent")]
    Consent,
}

impl OAuth2DiscordPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2DiscordPrompt::None => "none",
            OAuth2DiscordPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for OAuth2DiscordPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
