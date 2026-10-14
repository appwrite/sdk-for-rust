use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2OktaPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "consent")]
    Consent,
}

impl OAuth2OktaPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2OktaPrompt::None => "none",
            OAuth2OktaPrompt::Login => "login",
            OAuth2OktaPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for OAuth2OktaPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
