use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2MicrosoftPrompt {
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

impl ProjectOAuth2MicrosoftPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2MicrosoftPrompt::None => "none",
            ProjectOAuth2MicrosoftPrompt::Login => "login",
            ProjectOAuth2MicrosoftPrompt::Consent => "consent",
            ProjectOAuth2MicrosoftPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2MicrosoftPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
