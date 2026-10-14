use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2ZohoPrompt {
    #[serde(rename = "consent")]
    #[default]
    Consent,
}

impl OAuth2ZohoPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2ZohoPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for OAuth2ZohoPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
