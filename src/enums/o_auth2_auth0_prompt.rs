use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2Auth0Prompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "consent")]
    Consent,
}

impl OAuth2Auth0Prompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2Auth0Prompt::None => "none",
            OAuth2Auth0Prompt::Login => "login",
            OAuth2Auth0Prompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for OAuth2Auth0Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
