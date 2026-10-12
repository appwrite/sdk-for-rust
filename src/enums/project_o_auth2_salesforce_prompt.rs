use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2SalesforcePrompt {
    #[serde(rename = "login")]
    #[default]
    Login,
    #[serde(rename = "consent")]
    Consent,
}

impl ProjectOAuth2SalesforcePrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2SalesforcePrompt::Login => "login",
            ProjectOAuth2SalesforcePrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2SalesforcePrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
