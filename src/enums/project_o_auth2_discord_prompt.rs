use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2DiscordPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "consent")]
    Consent,
}

impl ProjectOAuth2DiscordPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2DiscordPrompt::None => "none",
            ProjectOAuth2DiscordPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2DiscordPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
