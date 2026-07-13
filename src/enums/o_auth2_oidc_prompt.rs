use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2OidcPrompt {
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

impl OAuth2OidcPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2OidcPrompt::None => "none",
            OAuth2OidcPrompt::Login => "login",
            OAuth2OidcPrompt::Consent => "consent",
            OAuth2OidcPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for OAuth2OidcPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
