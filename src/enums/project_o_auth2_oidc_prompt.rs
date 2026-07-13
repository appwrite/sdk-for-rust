use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2OidcPrompt {
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

impl ProjectOAuth2OidcPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2OidcPrompt::None => "none",
            ProjectOAuth2OidcPrompt::Login => "login",
            ProjectOAuth2OidcPrompt::Consent => "consent",
            ProjectOAuth2OidcPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2OidcPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
