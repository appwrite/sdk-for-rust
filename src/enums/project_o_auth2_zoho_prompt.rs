use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2ZohoPrompt {
    #[serde(rename = "consent")]
    #[default]
    Consent,
}

impl ProjectOAuth2ZohoPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2ZohoPrompt::Consent => "consent",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2ZohoPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
