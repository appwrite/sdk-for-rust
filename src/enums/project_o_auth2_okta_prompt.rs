use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2OktaPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "consent")]
    Consent,
}

impl ProjectOAuth2OktaPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2OktaPrompt::None => "none",
            ProjectOAuth2OktaPrompt::Login => "login",
            ProjectOAuth2OktaPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2OktaPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
