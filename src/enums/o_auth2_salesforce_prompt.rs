use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2SalesforcePrompt {
    #[serde(rename = "login")]
    #[default]
    Login,
    #[serde(rename = "consent")]
    Consent,
}

impl OAuth2SalesforcePrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2SalesforcePrompt::Login => "login",
            OAuth2SalesforcePrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for OAuth2SalesforcePrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
