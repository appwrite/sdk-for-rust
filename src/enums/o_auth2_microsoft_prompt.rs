use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2MicrosoftPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "consent")]
    Consent,
    #[serde(rename = "select_account")]
    SelectAccount,
}

impl OAuth2MicrosoftPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2MicrosoftPrompt::None => "none",
            OAuth2MicrosoftPrompt::Login => "login",
            OAuth2MicrosoftPrompt::Consent => "consent",
            OAuth2MicrosoftPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for OAuth2MicrosoftPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
